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
use xxai::{bin_u64, u64_bin};
use xxpg::Q;

use crate::{
  es,
  es::KIND_SYNC_FAV,
  kv::sync::{has_more, set_last},
  K,
};

const LIMIT: usize = 2048;

Q! {
fav_li:
SELECT id,cid,rid,ts,aid FROM fav.user WHERE uid=$1 AND id>$2 ORDER BY id LIMIT 2048;

}
// fav_ym_n: "SELECT TO_CHAR(to_timestamp(cast(ts/1000 as u64)) AT TIME ZONE 'UTC','YYYYMM')::u64 AS ym, COUNT(1)::u64 FROM fav.user WHERE uid=$1 GROUP BY ym"

macro_rules! es_sync {
  ($uid:expr, $channel_id: expr, $li: expr) => {
    trt::spawn!({
      let channel_id = $channel_id;
      let uid = $uid;
      let uid_bin = u64_bin(uid);
      let fav_id = $li[0];
      if let Some(last_fav_id) = has_more(K::FAV_LAST, &uid_bin, fav_id).await? {
        let mut id = fav_id;
        loop {
          let prev_id = id;
          let fav_li = fav_li(uid, id).await?;
          let len = fav_li.len();
          if len > 0 {
            id = fav_li.last().unwrap().0;
            let mut json = String::new();
            for i in &fav_li {
              json += &format!(",{},{},{},{}", i.1, i.2, i.3, i.4);
            }
            es::publish_b64(
              &channel_id,
              KIND_SYNC_FAV,
              format!("{uid},{prev_id},{id}{json}"),
            )
            .await?;
          }
          if len != LIMIT {
            break;
          }
        }
        if id != last_fav_id {
          set_last(K::FAV_LAST, uid, id);
        }
      }
    });
  };
}

pub async fn get(client: Client, Path(li): Path<String>) -> awp::Result<Response> {
  let li = xxai::b64_u64_li(li);
  if li.len() >= 2 {
    let uid = li[0];
    if client.is_login(uid).await? {
      let client_id = u64_bin(client.id);
      let channel_id = xxai::b64(&client_id[..]);

      trt::spawn!({
        KV.zadd(
          &*K::nchan(uid),
          None,
          None,
          false,
          false,
          (xxai::now() as f64, &client_id[..]),
        )
        .await?;
      });

      let url = format!("/nchan/{}", channel_id);

      es_sync!(uid, channel_id, &li[1..]);

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
