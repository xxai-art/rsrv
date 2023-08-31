use std::collections::HashMap;

use anyhow::Result;
use axum::body::Bytes;
use client::Client;
use tokio::sync::OnceCell;
use x0::fred::types::Script;
use xxai::z85_decode_u64_li;

use crate::C::action::{CLICK, FAV, FAV_RM};

static REC_ACTION: [u8; 3] = [CLICK, FAV, FAV_RM];

static QID: OnceCell<Script> = OnceCell::const_new();

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
        script.load(kv).await.unwrap();
        script
      })
      .await
      .evalsha(kv, vec!["id", "q"], q.as_ref())
      .await?,
  )
}

pub async fn rec_by_action(cid_rid_action: HashMap<(u8, u64), u8>) -> Result<Vec<u64>> {
  if cid_rid_action.is_empty() {
    return Ok(vec![]);
  }
  dbg!("TODO rec_by_action", cid_rid_action);
  let rec = Vec::with_capacity(64);
  Ok(rec)
}

#[derive(Debug, serde::Deserialize)]
struct Req {
  level: u8,
  all: Vec<Vec<String>>,
}

pub async fn post(mut client: Client, body: Bytes) -> awp::any!() {
  let mut rec_action = HashMap::default();
  if let Some(uid) = client.uid().await? {
    let ts = sts::ms();
    let req: Req = serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;
    dbg!(req.level);
    let all = req.all;
    let mut to_insert = Vec::with_capacity(all.iter().map(|v| v.len()).sum());
    for li in all {
      if !li.is_empty() {
        let q = xxai::str::low_short(&li[0]);
        let (qid, new) = qid(&q).await?;
        if new {
          trt::spawn!({
            gt::QE(format!("INSERT INTO q (id,q) VALUES ({qid},$1)"), &[&q]).await?;
          });
        }
        for cid_rid_li in &li[1..] {
          let cid_rid_li = z85_decode_u64_li(cid_rid_li)?;
          if !cid_rid_li.is_empty() {
            let action = cid_rid_li[0];
            for cid_rid in cid_rid_li[1..].chunks(2) {
              let cid = cid_rid[0];
              let rid = cid_rid[1];
              let action = action as u8;
              let cid = cid as u8;
              if REC_ACTION.contains(&action) {
                rec_action.insert((cid, rid), action);
              }
              to_insert.push(format!("({uid},{action},{cid},{rid},{qid},{ts})"));
            }
          }
        }
      }
    }
    if !to_insert.is_empty() {
      trt::spawn!({
        let to_insert = to_insert.join(",");
        gt::QE(
          format!("INSERT INTO log (uid,aid,cid,rid,q,ts) VALUES {to_insert}"),
          &[],
        )
        .await?;
      });
    }
  }
  Ok(rec_by_action(rec_action).await?)
}
