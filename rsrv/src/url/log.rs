use std::collections::HashSet;

use anyhow::Result;
use axum::body::Bytes;
use client::Client;
use gt::GE;
use serde_json::Value;
use tokio::sync::OnceCell;
use x0::fred::types::Script;
use xxai::{u64_bin, z85_decode_u64_li};
use xxpg::ToSql;

use crate::{
  db::seen,
  es::{publish_to_user_client, KIND_SYNC_SEEN},
  kv::sync::{has_more, set_last},
  K,
};

const QID: OnceCell<Script> = OnceCell::const_new();

pub async fn qid(q: impl AsRef<str>) -> Result<(u64, bool)> {
  let kv = x0::KV.0.get().unwrap();
  Ok(
    QID
      .get_or_init(|| async {
        let script = Script::from_lua(
          r#"local idKey = KEYS[1]
local qKey = KEYS[2]
local q = ARGV[1]

local id = redis.call("HGET", qKey, q)

if id then
  return {id,0}
end

id = redis.call("HINCRBY", idKey, "q", 1)
redis.call("HSET", qKey, q, id)
return {id,1}"#,
        );
        let _ = script.load(kv).await.unwrap();
        script
      })
      .await
      .evalsha(kv, vec!["id", "q"], q.as_ref())
      .await?,
  )
}

// fn log(uid: u64, q: String, action: u64, cid: u64, rid: u64) {
//   trt::spawn!({
//     let q = xxai::str::low_short(q);
//     GQ(format!("INSERT INTO log (ts,) VALUES"), &[]).await?;
//     dbg!(new, uid, q, qid, action, cid, rid);
//   });
// }

pub async fn post(mut client: Client, body: Bytes) -> awp::any!() {
  let mut r = Vec::new();
  if let Some(uid) = client.uid().await? {
    let ts = xxai::time::sec();
    let all: Vec<Vec<String>> =
      serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;

    let mut to_insert = Vec::with_capacity(all.iter().map(|v| v.len()).sum());
    for li in all {
      if !li.is_empty() {
        let q = xxai::str::low_short(&li[0]);
        let (qid, new) = qid(&q).await?;
        if new {
          trt::spawn!({
            GE(format!("INSERT INTO q (id,q) VALUES ({qid},$1)"), &[&q]).await?;
          });
        }
        for cid_rid_li in &li[1..] {
          let cid_rid_li = z85_decode_u64_li(cid_rid_li)?;
          if !cid_rid_li.is_empty() {
            let action = cid_rid_li[0];
            for cid_rid in (&cid_rid_li[1..]).chunks(2) {
              let cid = cid_rid[0];
              let rid = cid_rid[1];
              to_insert.push(format!("({uid},{action},{cid},{rid},{qid},{ts})"));
            }
          }
        }
      }
    }
    if !to_insert.is_empty() {
      trt::spawn!({
        let to_insert = to_insert.join(",");
        GE(
          format!("INSERT INTO log (uid,aid,cid,rid,q,ts) VALUES {to_insert}"),
          &[],
        )
        .await?;
      });
    }
  }
  // todo 根据用户的行为，往r中放入新的推荐
  Ok(r)
}
