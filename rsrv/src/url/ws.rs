use std::collections::HashMap;

use anyhow::Result;
use awp::ok;
use axum::{
  body::Bytes,
  extract::Path,
  http::{header::HeaderMap, StatusCode},
  response::{IntoResponse, Response},
};
use client::Client;
use int_enum::IntEnum;
use intbin::u64_bin;
use paste::paste;
use ub64::{b64d, b64e};

use crate::{
  kv::sync::{has_more, set_last},
  ws,
  C::{self, WR},
  K,
};

// macro_rules! json {
//   (fav, $prev_id:ident,$last_id:ident,$str:ident, $li:expr) => {{
//     $str += &format!(",{},{}", $prev_id, $last_id);
//     for i in $li {
//       $str += &format!(",{},{},{},{}", i.1, i.2, i.3, i.4)
//     }
//   }};
//   (seen, $prev_id:ident, $last_id:ident, $str:ident, $li:expr) => {{
//     let li = $li;
//     let mut map = HashMap::new();
//     for i in li {
//       map.entry(i.1).or_insert_with(Vec::new).push((i.0, i.2));
//     }
//     $str += &format!(",{}", $prev_id);
//     for (cid, ts_rid_li) in map {
//       let mut li = Vec::with_capacity(ts_rid_li.len() * 2 + 1);
//       let mut base = 0;
//       for (ts, rid) in ts_rid_li {
//         let ts = ts as u64;
//         li.push(ts - base);
//         base = ts;
//         li.push(rid as _);
//       }
//       li.push(cid as u64);
//       $str += &format!(",\"{}\"", xxai::z85_encode_u64_li(li));
//     }
//   }};
// }
//
// macro_rules! es_sync {
//   ($uid:expr, $channel_id: expr, $prev_id: expr, $key:ident) => {{
//     let channel_id = $channel_id.clone();
//     let prev_id = $prev_id;
//     let uid = $uid;
//     trt::spawn!({
//       let uid_bin = u64_bin(uid);
//       paste! {
//           let last_key = K::[< $key:upper _LAST >];
//           let has_more = has_more(last_key, &uid_bin, prev_id).await?;
//           if has_more.more {
//               let mut id = prev_id;
//               loop {
//                   let li = [<$key _li>](uid, id).await?;
//                   let len = li.len();
//                   if len > 0 {
//                       let mut json = String::new();
//                       let last_id = li.last().unwrap().0;
//                       json!($key,id,last_id,json,&li);
//                       ws::send(
//                           &channel_id,
//                           ws::[<KIND_SYNC_ $key:upper>],
//                           format!("{uid}{json}"),
//                       ).await?;
//                       id = last_id;
//                   }
//                   if len != LIMIT {
//                       break;
//                   }
//               }
//               if id != has_more.id {
//                   set_last(last_key, uid, id);
//               }
//           }
//       }
//     });
//   }};
// }
//
// macro_rules! es_sync_li {
//     ($uid:expr, $channel_id:expr, $li:ident : $($pos:expr,$key:ident);*) => {
//         $(
//             es_sync!($uid, $channel_id, $li[$pos], $key);
//         )*
//     };
//     ($uid:expr, $channel_id: expr, $li: expr) => {{
//         let li = $li;
//         es_sync_li!($uid, $channel_id, li : 0, fav; 1, seen);
//     }}
// }

async fn is_login(
  client: &Client,
  uid: u64,
  channel_id: impl Into<String>,
) -> anyhow::Result<bool> {
  if client.is_login(uid).await? {
    return Ok(true);
  }
  let channel_id = channel_id.into();
  trt::spawn!({
    ws::send(channel_id, C::WS::未登录, uid).await?;
  });
  Ok(false)
}

pub async fn post(
  client: Client,
  Path(channel_id): Path<String>,
  body: Bytes,
) -> awp::Result<Response> {
  if !body.is_empty() {
    let uid_client_id = vb::d(b64d(&channel_id)?)?;
    if uid_client_id.len() == 2 {
      let uid = uid_client_id[0];
      // let client_id = uid_client_id[1];

      if is_login(&client, uid, &channel_id).await? {
        let action = C::WR::from_int(body[0])?;
        crate::db::ws(action, uid, channel_id, &body[1..]).await?;
      }
    }
  }
  ok()
}

pub async fn get(client: Client, Path(uid): Path<String>) -> awp::Result<Response> {
  let uid = ub64::b64_u64(uid);
  let client_id = vb::e(&[uid, client.id]);
  let channel_id = b64e(&client_id[..]);
  let url = format!("/nchan/{}", &channel_id);

  if is_login(&client, uid, channel_id).await? {
    trt::spawn!({
      KV.zadd(
        &*K::nchan(uid),
        None,
        None,
        false,
        false,
        (xxai::now() as f64, u64_bin(client.id)),
      )
      .await?;
    });
  }
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
