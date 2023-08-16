use std::collections::HashSet;

use axum::body::Bytes;
use client::Client;
use gt::GQ;
use serde_json::Value;
use xxai::{u64_bin, z85_decode_u64_li};

use crate::{
  db::seen,
  es::{publish_to_user_client, KIND_SYNC_SEEN},
  kv::sync::{has_more, set_last},
  K,
};

fn log(uid: u64, q: String, action: u64, cid: u64, rid: u64) {
  trt::spawn!({
    dbg!(uid, q, action, cid, rid);
  });
}

pub async fn post(mut client: Client, body: Bytes) -> awp::any!() {
  let mut r = Vec::new();
  if let Some(uid) = client.uid().await? {
    let all: Vec<Vec<String>> =
      serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;

    for li in all {
      if !li.is_empty() {
        let q = &li[0];
        for cid_rid_li in &li[1..] {
          let cid_rid_li = z85_decode_u64_li(cid_rid_li)?;
          if !cid_rid_li.is_empty() {
            let action = cid_rid_li[0];
            for cid_rid in (&cid_rid_li[1..]).chunks(2) {
              let cid = cid_rid[0];
              let rid = cid_rid[1];
              log(uid, q.to_string(), action, cid, rid);
            }
          }
        }
      }
    }
  }
  // if li.len() > 2 {
  //   if let Some(uid) = li[0].as_u64() {
  //     if client.is_login(uid).await? {
  //       if let Some(last_sync_id) = li[1].as_u64() {
  //         let mut to_insert = Vec::new();
  //         let mut to_publish = Vec::new();
  //         let mut ts = xxai::time::ms();
  //         let uid_bin = u64_bin(uid);
  //
  //         for i in &li[2..] {
  //           if let Some(cid_rid_li) = i.as_str() {
  //             let cid_rid_li = z85_decode_u64_li(cid_rid_li)?;
  //             let cid = cid_rid_li[0];
  //             let mut rid_set = HashSet::with_capacity(cid_rid_li.len() - 1);
  //
  //             let mut pre = 0;
  //             for i in &cid_rid_li[1..] {
  //               pre += i;
  //               rid_set.insert(pre);
  //             }
  //
  //             if !rid_set.is_empty() {
  //               let rid_in = rid_set
  //                 .iter()
  //                 .map(|x| x.to_string())
  //                 .collect::<Vec<String>>()
  //                 .join(",");
  //
  //               for i in GQ(
  //                 &format!(
  //                   "SELECT rid FROM seen WHERE uid={uid} AND cid={cid} AND rid IN ({rid_in})"
  //                 ),
  //                 &[],
  //               )
  //               .await?
  //               {
  //                 let rid: i64 = i.get(0);
  //                 rid_set.remove(&(rid as u64));
  //               }
  //             }
  //             if !rid_set.is_empty() {
  //               let mut publish = Vec::with_capacity(rid_set.len() + 1);
  //               for rid in rid_set {
  //                 publish.push(rid);
  //                 to_insert.push(format!("({uid},{cid},{rid},{ts})"));
  //                 ts += 1;
  //               }
  //               xxai::diffli(&mut publish);
  //
  //               publish.push(cid);
  //               let publish = xxai::z85_encode_u64_li(publish);
  //               to_publish.push(format!("\"{publish}\""));
  //             }
  //           }
  //         }
  //
  //         let to_insert_is_empty = to_insert.is_empty();
  //
  //         let has_more = has_more(K::SEEN_LAST, uid_bin, last_sync_id).await?;
  //         let prev_id = has_more.id;
  //         if has_more.more {
  //           let seen_li = seen::after_ts(seen::after_ts_sql(uid, last_sync_id)).await?;
  //           for i in seen_li {
  //             r.push(i);
  //           }
  //           if to_insert_is_empty {
  //             r.push(prev_id);
  //           }
  //         } else if to_insert_is_empty {
  //           r.push(last_sync_id);
  //         }
  //
  //         if !to_insert_is_empty {
  //           ts -= 1;
  //           let to_insert = to_insert.join(",");
  //           GQ(
  //             &format!("INSERT INTO seen (uid,cid,rid,ts) VALUES {to_insert}"),
  //             &[],
  //           )
  //           .await?;
  //           set_last(K::SEEN_LAST, uid, ts);
  //           let to_publish = to_publish.join(",");
  //           let diff = ts - has_more.id;
  //           publish_to_user_client(
  //             client.id,
  //             uid,
  //             KIND_SYNC_SEEN,
  //             format!("{prev_id},{diff},{to_publish}"),
  //           );
  //           r.push(ts);
  //         }
  //       }
  //     }
  //   }
  // }
  Ok(r)
}
