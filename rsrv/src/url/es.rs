use axum::{
  extract::Path,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use client::Client;
use x0::{
  fred::interfaces::{HashesInterface, SortedSetsInterface},
  KV,
};
use xxai::u64_bin;
use xxpg::Q;

use crate::{es, es::KIND_SYNC_FAV, K};

const LIMIT: usize = 1024;

Q!(

    fav_li:
    SELECT id,cid,rid,ctime,action FROM fav.user WHERE user_id=$1 AND id>$2 ORDER BY id LIMIT 1024

);

macro_rules! es_sync {
  ($user_id:expr, $channel_id: expr, $li: expr) => {
    trt::spawn!({
      let channel_id = $channel_id;
      let user_id = $user_id;
      let p = KV.pipeline();
      p.hincrby(K::FAV_ID, user_id, 0).await?;
      p.hincrby(K::FAV_SUM, user_id, 0).await?;
      let r: Vec<u64> = p.all().await?;

      let fav_synced_id = $li[0];
      let _fav_synced = $li[1];
      let mut n = 0;

      if fav_synced_id < r[0] {
        let mut id = fav_synced_id;
        loop {
          let fav_li = fav_li(&user_id, &id).await?;
          let len = fav_li.len();
          n += len;
          if len > 0 {
            id = fav_li.last().unwrap().0;
            let mut json = String::new();
            for i in &fav_li {
              json += &format!("{},{},{},{},", i.1, i.2, i.3, i.4);
            }
            let json = &json[..json.len() - 1];
            es::publish_b64(&channel_id, KIND_SYNC_FAV, format!("{user_id},{json},{id}"));
          }
          if len != LIMIT {
            break;
          }
        }
      }

      dbg!(n, r[1]);
    });
  };
}

pub async fn get(client: Client, Path(li): Path<String>) -> awp::Result<Response> {
  let li = xxai::b64_u64_li(li);
  if li.len() >= 3 {
    let user_id = li[0];

    if client.is_login(user_id).await? {
      let client_id = u64_bin(client.id);
      let channel_id = xxai::b64(&client_id[..]);

      trt::spawn!({
        KV.zadd(
          &*K::nchan(user_id),
          None,
          None,
          false,
          false,
          (xxai::now() as f64, &client_id[..]),
        )
        .await?;
      });

      let url = format!("/nchan/{}", channel_id);

      es_sync!(user_id, channel_id, &li[1..]);

      return Ok(
        (
          StatusCode::OK,
          [
            ("X-Accel-Redirect", url.as_str()),
            ("X-Accel-Buffering", "no"),
          ],
          "",
        )
          .into_response(),
      );
    }
  }

  Ok((StatusCode::UNAUTHORIZED, "").into_response())
}
