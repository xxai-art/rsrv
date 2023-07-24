use std::collections::HashSet;

use anyhow::Result;
use axum::body::Bytes;
use client::Client;
use gt::GQ;
use serde_json::Value;
use x0::{fred::interfaces::HashesInterface, KV};
use xxai::u64_bin;

use crate::{
  es::{publish_to_user_client, KIND_SYNC_SEEN},
  kv::sync::{has_more, set_last},
  K,
};

/*
CREATE TABLE IF NOT EXISTS seen (
uid BIGINT NULL,
cid TINYINT NULL,
rid BIGINT NULL,
ts TIMESTAMP(3) NOT NULL,
TIME INDEX (ts),
PRIMARY KEY (uid, cid, rid)
)
ENGINE=mito
WITH(
regions = 1
)
*/

pub async fn seen_after_ts(uid: u64, ts: u64) -> Result<Vec<u64>> {
  let mut r = Vec::new();
  for i in GQ(
    &format!("SELECT cid,rid,CAST(ts as BIGINT) t FROM seen WHERE uid={uid} AND ts>{ts}"),
    &[],
  )
  .await?
  {
    let cid: i8 = i.get(0);
    r.push(cid as u64);
    let rid: i64 = i.get(1);
    r.push(rid as u64);
    let ts: i64 = i.get(2);
    r.push(ts as u64);
  }
  Ok(r)
}

pub async fn post(client: Client, body: Bytes) -> awp::any!() {
  let mut r = Vec::new();
  let li: Vec<Value> = serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;

  if li.len() > 2 {
    if let Some(uid) = li[0].as_u64() {
      if client.is_login(uid).await? {
        if let Some(last_sync_id) = li[1].as_u64() {
          let mut to_insert = Vec::new();
          let mut to_publish = Vec::new();
          let mut ts = xxai::time::ms();
          let uid_bin = u64_bin(uid);

          for i in &li[2..] {
            if let Some(cid_rid_li) = i.as_array() {
              if let Some(cid) = cid_rid_li[0].as_u64() {
                let mut rid_set = HashSet::with_capacity(cid_rid_li.len() - 1);

                for i in &cid_rid_li[1..] {
                  if let Some(i) = i.as_u64() {
                    rid_set.insert(i);
                  }
                }

                if !rid_set.is_empty() {
                  let rid_in = rid_set
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                  for i in GQ(
                    &format!(
                      "SELECT rid FROM seen WHERE uid={uid} AND cid={cid} AND rid IN ({rid_in})"
                    ),
                    &[],
                  )
                  .await?
                  {
                    rid_set.remove(&i.get(0));
                  }
                }
                if !rid_set.is_empty() {
                  let mut publish = Vec::with_capacity(rid_set.len() + 1);
                  publish.push(cid);
                  for rid in rid_set {
                    to_insert.push(format!("({uid},{cid},{rid},{ts})"));
                    ts += 1;
                  }
                  let publish = publish
                    .into_iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                  to_publish.push(format!("[{publish}]"));
                }
              }
            }
          }

          if has_more(K::SEEN_LAST, uid_bin, last_sync_id)
            .await?
            .is_some()
          {
            for i in seen_after_ts(uid, last_sync_id).await? {
              r.push(i);
            }
          }

          if !to_insert.is_empty() {
            ts -= 1;
            let to_insert = to_insert.join(",");
            GQ(
              &format!("INSERT INTO seen (uid,cid,rid,ts) VALUES {to_insert}"),
              &[],
            )
            .await?;
            set_last(K::SEEN_LAST, uid, ts);
            let to_publish = to_publish.join(",");
            publish_to_user_client(client.id, uid, KIND_SYNC_SEEN, format!("[{to_publish}]"));
            r.push(ts);
          }
        }
      }
    }
  }
  dbg!(&r);
  Ok(r)
}
