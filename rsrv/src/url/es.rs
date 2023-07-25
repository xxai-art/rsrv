use axum::{
  extract::Path,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use client::Client;
use paste::paste;
use x0::{fred::interfaces::SortedSetsInterface, KV};
use xxai::u64_bin;
use xxpg::Q;

use crate::{
  es,
  es::KIND_SYNC_FAV,
  kv::sync::{has_more, set_last},
  K,
};

const LIMIT: usize = 2048;

Q! {
    fav_li:SELECT id,cid,rid,ts,aid FROM fav.user WHERE uid=$1 AND id>$2 ORDER BY id LIMIT 2048;
}

// fav_ym_n: "SELECT TO_CHAR(to_timestamp(cast(ts/1000 as u64)) AT TIME ZONE 'UTC','YYYYMM')::u64 AS ym, COUNT(1)::u64 FROM fav.user WHERE uid=$1 GROUP BY ym"

macro_rules! json_fav {
  ($i:expr) => {{
    let i = $i;
    format!(",{},{},{},{}", i.1, i.2, i.3, i.4)
  }};
}

macro_rules! es_sync {
  ($uid:expr, $channel_id: expr, $prev_id: expr, $key:ident) => {
    trt::spawn!({
      let channel_id = $channel_id;
      let uid = $uid;
      let uid_bin = u64_bin(uid);
      paste! {
      let last_key = K::[< $key:upper _LAST >];
      if let Some(last_id) = has_more(last_key, &uid_bin, $prev_id).await? {
        let mut id = $prev_id;
        loop {
          let prev_id = id;
          let li = [<$key _li>](uid, id).await?;
          let len = li.len();
          if len > 0 {
            id = li.last().unwrap().0;
            let mut json = String::new();
            for i in &li {
              json += &[<json_ $key>]!(i);
            }
            es::publish_b64(
              &channel_id,
              [<KIND_SYNC_ $key:upper>],
              format!("{uid},{prev_id},{id}{json}"),
            )
            .await?;
          }
          if len != LIMIT {
            break;
          }
        }
        if id != last_id {
          set_last(last_key, uid, id);
        }
      }
      }
    });
  };
}

macro_rules! es_sync_li {
  ($uid:expr, $channel_id: expr, $li: expr) => {
    es_sync!($uid, $channel_id, $li[0], fav)
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

      es_sync_li!(uid, channel_id, &li[1..]);

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
