use std::collections::HashSet;

use axum::body::Bytes;
use client::Client;
use intbin::u64_bin;
use sonic_rs::Value;
use xxai::z85_decode_u64_li;

use crate::{
  db::seen,
  kv::sync::{has_more, set_last},
  ws::send_user,
  C::WS::浏览,
  K,
};

pub async fn post(client: Client, body: Bytes) -> awp::any!() {
  let mut r = Vec::new();
  let li: Vec<Value> = sonic_rs::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;

  if li.len() > 2 {
    if let Some(uid) = li[0].as_u64() {
      if client.is_login(uid).await? {
        if let Some(last_sync_id) = li[1].as_u64() {
          let mut to_insert = Vec::new();
          let mut to_publish = Vec::new();
          let mut ts = sts::ms();
          let uid_bin = u64_bin(uid);

          for i in &li[2..] {
            if let Some(cid_rid_li) = i.as_str() {
              let cid_rid_li = z85_decode_u64_li(cid_rid_li)?;
              let cid = cid_rid_li[0];
              let mut rid_set = HashSet::with_capacity(cid_rid_li.len() - 1);

              let mut pre = 0;
              for i in &cid_rid_li[1..] {
                pre += i;
                rid_set.insert(pre);
              }

              if !rid_set.is_empty() {
                let rid_in = rid_set
                  .iter()
                  .map(|x| x.to_string())
                  .collect::<Vec<String>>()
                  .join(",");

                for i in gt::Q(
                  format!(
                    "SELECT rid FROM seen WHERE uid={uid} AND cid={cid} AND rid IN ({rid_in})"
                  ),
                  &[],
                )
                .await?
                {
                  let rid: i64 = i.get(0);
                  rid_set.remove(&(rid as u64));
                }
              }
              if !rid_set.is_empty() {
                let mut publish = Vec::with_capacity(rid_set.len() + 1);
                for rid in rid_set {
                  publish.push(rid);
                  to_insert.push(format!("({uid},{cid},{rid},{ts})"));
                  ts += 1;
                }
                xxai::diffli(&mut publish);

                publish.push(cid);
                let publish = xxai::z85_encode_u64_li(publish);
                to_publish.push(format!("\"{publish}\""));
              }
            }
          }

          let to_insert_is_empty = to_insert.is_empty();

          let has_more = has_more(K::SEEN_LAST, uid_bin, last_sync_id).await?;
          let prev_id = has_more.id;
          if has_more.more {
            let seen_li = seen::after_ts(seen::after_ts_sql(uid, last_sync_id)).await?;
            for i in seen_li {
              r.push(i);
            }
            if to_insert_is_empty {
              r.push(prev_id);
            }
          } else if to_insert_is_empty {
            r.push(last_sync_id);
          }

          if !to_insert_is_empty {
            ts -= 1;
            let to_insert = to_insert.join(",");
            gt::QE(
              format!("INSERT INTO seen (uid,cid,rid,ts) VALUES {to_insert}"),
              &[],
            )
            .await?;
            set_last(K::SEEN_LAST, uid, ts);
            let to_publish = to_publish.join(",");
            let diff = ts - has_more.id;
            send_user(
              uid,
              client.id,
              浏览,
              format!("{prev_id},{diff},{to_publish}"),
            );
            r.push(ts);
          }
        }
      }
    }
  }
  Ok(r)
}
