use axum::body::Bytes;
use client::Client;
use serde_json::Value;
use x0::{fred::interfaces::HashesInterface, KV};
use xxai::u64_bin;
use xxpg::{Q, Q01};

// use crate::{
//     es::{publish_to_user_client, KIND_SYNC_FAV},
//     K,
// };

pub async fn post(client: Client, body: Bytes) -> awp::any!() {
  let mut r = Vec::new();
  let li: Vec<Value> = serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;

  if li.len() > 2 {
    if let Some(uid) = li[0].as_u64() {
      if client.is_login(uid).await? {
        let mut s = String::new();
        let mut ts = xxai::time::ms();
        if let Some(last_sync_id) = li[1].as_u64() {
          for i in &li[2..] {
            if let Some(cid_rid_li) = i.as_array() {
              if let Some(cid) = cid_rid_li[0].as_u64() {
                for i in &cid_rid_li[1..] {
                  dbg!(&i);
                  "SELECT rid FROM seen WHERE uid={uid} AND cid=2 AND rid in (3,4)";
                }
              }
            }
          }
        }
      }
    }
    //   if client.is_login(uid).await? {
    //     let last_sync_id = li[1];
    //     let li = &li[2..];
    //     dbg!(uid, last_sync_id, li);
    //         let li: Vec<_> = li[2..]
    //             .chunks_exact(3)
    //             .map(|i| (i[0] as u16, i[1], i[2] as i8))
    //             .collect();
    //
    //         for i in &li {
    //             fav_rm(uid, i.0, i.1).await?
    //         }
    //
    //         let fav_li = fav_li(uid, last_sync_id).await?;
    //         let mut id = 0;
    //         if !fav_li.is_empty() {
    //             id = fav_li.last().unwrap().0;
    //             for i in fav_li {
    //                 r.push(i.1 as u64);
    //                 r.push(i.2);
    //                 r.push(i.3);
    //                 r.push(i.4 as u64);
    //             }
    //         }
    //
    //         let last_id = fav_batch_add(last_sync_id, client.id, uid, li).await?;
    //         if last_id != 0 {
    //             id = last_id;
    //         }
    //
    //         if id != 0 {
    //             r.push(id);
    //             kv_hset_fav_last(uid, id);
    //         }
    //   };
  }
  Ok(r)
}

// pub fn kv_hset_fav_last(uid: u64, id: u64) {
//     trt::spawn!({
//         KV.hset(K::FAV_LAST, (u64_bin(uid), u64_bin(id))).await?;
//     });
// }
