use anyhow::Result;
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
  db, es,
  kv::sync::{has_more, set_last},
  K,
};

const LIMIT: usize = 2048;

Q! {
    fav_li:SELECT id,cid,rid,ts,aid FROM fav.user WHERE uid=$1 AND id>$2 ORDER BY id LIMIT 2048;
}

async fn seen_li(uid: u64, ts: u64) -> Result<Vec<u64>> {
  let sql = &format!("SELECT cid,rid,CAST(ts as BIGINT) t FROM seen WHERE uid={uid} AND ts>CAST({ts} as TIMESTAMP) ORDER BY ts LIMIT 3");
  db::seen::after_ts(sql).await
}

macro_rules! last_0 {
  ($last:expr) => {
    $last.0
  };
}

macro_rules! last {
  ($last:expr) => {
    *$last
  };
}

macro_rules! json_seen {
  ($i:expr) => {{
    format!(",{}", $i)
  }};
}

macro_rules! json_fav {
  ($i:expr) => {{
    let i = $i;
    format!(",{},{},{},{}", i.1, i.2, i.3, i.4)
  }};
}

macro_rules! es_sync {
  ($uid:expr, $channel_id: expr, $prev_id: expr, $key:ident, $last:ident) => {{
    let channel_id = $channel_id.clone();
    let prev_id = $prev_id;
    let uid = $uid;
    trt::spawn!({
      let uid_bin = u64_bin(uid);
      paste! {
          let last_key = K::[< $key:upper _LAST >];
          if let Some(last_id) = has_more(last_key, &uid_bin, prev_id).await? {
              let mut id = prev_id;
              loop {
                  let prev_id = id;
                  let li = [<$key _li>](uid, id).await?;
                  let len = li.len();
                  if len > 0 {
                      id = $last!(li.last().unwrap());
                      let mut json = String::new();
                      for i in &li {
                          json += &[<json_ $key>]!(i);
                      }
                      es::publish_b64(
                          &channel_id,
                          es::[<KIND_SYNC_ $key:upper>],
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
  }};
}

macro_rules! es_sync_li {
  ($uid:expr, $channel_id: expr, $li:ident: $($pos:expr,$key:ident,$last:ident);*) => {
    $(
      es_sync!($uid, $channel_id, $li[$pos], $key, $last);
    )*
  };
  ($uid:expr, $channel_id: expr, $li: expr) => {{
    let li = $li;
    es_sync_li!($uid, $channel_id, li: 0, fav, last_0; 1, seen, last);
  }}
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
