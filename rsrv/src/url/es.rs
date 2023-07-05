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

use crate::{es, K};

const LIMIT: u64 = 2;

Q!(

fav_li:
  SELECT id,cid,rid,ctime,action FROM fav.user WHERE user_id=$1 AND id>$2 ORDER BY id LIMIT 2

);

macro_rules! es_sync {
  ($user_id:expr, $channel_id: expr, $li: expr) => {
    tokio::spawn(async move {
      let user_id = $user_id;
      let p = KV.pipeline();
      p.hincrby(K::FAV_ID, user_id, 0).await?;
      p.hincrby(K::FAV_SUM, user_id, 0).await?;
      let r: Vec<u64> = p.all().await?;

      let fav_synced_id = $li[0];
      let fav_synced = $li[1];

      if fav_synced_id < r[0] {
        let mut n = 0;
        let mut id = fav_synced_id;
        fav_li(&user_id, &fav_synced_id).await?;
      }

      es::publish_b64($channel_id, "good s");
      Ok::<_, anyhow::Error>(())
    });
  };
}

pub async fn get(mut client: Client, Path(li): Path<String>) -> awp::Result<Response> {
  let li = xxai::b64_u64_li(li);
  if li.len() >= 3 {
    let user_id = li[0];

    if client.is_login(user_id).await? {
      let client_id = u64_bin(client.id);

      KV.zadd(
        &[&b"nchan:"[..], &u64_bin(user_id)].concat()[..],
        None,
        None,
        false,
        false,
        (xxai::now() as f64, &client_id[..]),
      )
      .await?;

      let channel_id = xxai::b64(client_id);

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
