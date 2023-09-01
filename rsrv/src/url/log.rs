use std::collections::HashMap;

use anyhow::Result;
use axum::body::Bytes;
use client::Client;
use serde_json::Value;
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

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct CidRid {
  cid: u8,
  rid: u64,
}

#[derive(Debug)]
pub struct RecChina {
  action: u8,
  chain: Vec<CidRid>,
}

pub async fn rec_by_action(cid_rid_action: HashMap<CidRid, RecChina>) -> Result<Vec<u64>> {
  if cid_rid_action.is_empty() {
    return Ok(vec![]);
  }
  dbg!("TODO rec_by_action", cid_rid_action);
  let rec = Vec::with_capacity(64);
  Ok(rec)
}

pub async fn post(mut client: Client, body: Bytes) -> awp::any!() {
  let mut rec_action = HashMap::default();
  if let Some(uid) = client.uid().await? {
    let ts = sts::ms();
    let req: Vec<Value> = serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;
    let level = req[0].as_u64().unwrap_or(0); // 内容分级
    let all: Vec<Vec<String>> = req[1..]
      .into_iter()
      .map(|i| {
        if let Some(li) = i.as_array() {
          li.into_iter()
            .map(|v| v.as_str().unwrap_or("").to_owned())
            .collect()
        } else {
          vec![]
        }
      })
      .collect();

    let mut to_insert = Vec::with_capacity(all.iter().map(|v| v.len()).sum());
    let mut rec_chain = Vec::new();

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
            let action = cid_rid_li[0] as u8;
            macro_rules! to_insert {
              ($cid_rid:expr) => {{
                let cid_rid = $cid_rid;
                let cid = cid_rid[0];
                let rid = cid_rid[1];
                to_insert.push(format!("({uid},{action},{cid},{rid},{qid},{ts})"));
              }};
            }
            if REC_ACTION.contains(&action) {
              let crl: Vec<_> = cid_rid_li[1..].chunks(2).collect();
              let len = crl.len();
              if len > 1 {
                let cid_rid = crl[0];
                let cid = cid_rid[0] as u8;
                let rid = cid_rid[1];
                let key = CidRid { cid, rid };
                if action == FAV_RM {
                  rec_action.remove(&key);
                } else {
                  to_insert!(cid_rid);

                  let mut chain = Vec::with_capacity(len);

                  crl[1..].into_iter().for_each(|cid_rid| {
                    let rcid = cid_rid[0] as u8;
                    let rrid = cid_rid[1];
                    // 推荐的其他数据都是前置推荐序列，不插入log表
                    rec_chain.push(format!("({uid},{action},{cid},{rid},{rcid},{rrid},{ts})"));
                    chain.push(CidRid {
                      cid: rcid,
                      rid: rrid,
                    });
                  });
                  rec_action.insert(key, RecChina { action, chain });
                }
              }
            } else {
              cid_rid_li[1..].chunks(2).for_each(|i| to_insert!(i));
            }
          }
        }
      }
    }

    macro_rules! insert {
      ($li:ident,$sql:expr) => {
        if !$li.is_empty() {
          trt::spawn!({
            let li = $li.join(",");
            gt::QE($sql + &li, &[]).await?;
          });
        }
      };
    }
    insert!(
      to_insert,
      "INSERT INTO log (uid,aid,cid,rid,q,ts) VALUES ".to_owned()
    );
    insert!(
      rec_chain,
      "INSERT INTO rec_chain (uid,aid,cid,rid,rcid,rrid,ts) VALUES ".to_owned()
    );
  }
  Ok(rec_by_action(rec_action).await?)
}
