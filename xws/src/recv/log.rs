use std::collections::HashSet;

use anyhow::Result;
use async_lazy::Lazy;
use msgpacker::prelude::*;
use x0::fred::types::Script;

use crate::r#type::AllWs;

#[derive(Debug, PartialEq, Eq, MsgPacker)]
struct Log {
  li: Vec<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq, MsgPacker)]
struct LogLi {
  li: Vec<Log>,
}

static LOG_CID: [u8; 1] = [CID_IMG];
static REC_ACTION: [u8; 2] = [CLICK, FAV];

static QID: Lazy<Script> = Lazy::const_new(|| {
  let kv = x0::KV.0.get().unwrap();
  Box::pin(async {
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
});

pub async fn _qid(q: impl AsRef<str>) -> Result<(u64, bool)> {
  let kv = x0::KV.0.get().unwrap();
  Ok(
    QID
      .force()
      .await
      .evalsha(kv, vec!["id", "q"], q.as_ref())
      .await?,
  )
}

async fn qid(q: impl AsRef<str>) -> Result<u64> {
  let q = xxai::str::low_short(q);
  let (id, new) = _qid(&q).await?;
  if new {
    trt::spawn!({
      gt::QE(format!("INSERT INTO q (id,q) VALUES ({id},$1)"), &[&q]).await?;
    });
  }
  Ok(id)
}

pub async fn log(uid: u64, level: u8, buf: &[u8], all_ws: AllWs) -> Result<()> {
  let ts = sts::ms();
  let (_, log_li) = LogLi::unpack(&buf)?;
  let log_li = log_li.li;

  let mut rec_action = Vec::new();
  let mut rec_chain = Vec::new();
  let mut exist = HashSet::new();
  let mut to_insert = Vec::with_capacity(log_li.iter().map(|v| v.li.len()).sum());

  for li in log_li {
    let li = li.li;
    if li.len() > 1 {
      let q = &li[0];

      let qid = qid(if q.is_empty() {
        "".into()
      } else {
        String::from_utf8_lossy(q)
      })
      .await?;

      for li in &li[1..] {
        if li.len() > 0 {
          let action = li[0];

          macro_rules! to_insert {
            ($cid:ident,$rid:ident) => {{
              let cid = $cid;
              let rid = $rid;
              to_insert.push(format!("({uid},{action},{cid},{rid},{qid},{ts})"));
            }};
          }
          if action == FAV_RM {
            cid_rid_li[1..].chunks(2).for_each(|i| {
              let cid = i[0] as u8;
              if LOG_CID.contains(&cid) {
                let rid = i[1];
                let key = (cid, rid);
                exist.remove(&key);
                to_insert!(cid, rid);
                rec_action.retain(|li: &Vec<(u8, u64)>| li[0] != key);
              }
            });
          }
        }
      }
    }
  }

  Ok(())
}
