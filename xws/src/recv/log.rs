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

pub async fn qid(q: impl AsRef<str>) -> Result<(u64, bool)> {
  let kv = x0::KV.0.get().unwrap();
  Ok(
    QID
      .force()
      .await
      .evalsha(kv, vec!["id", "q"], q.as_ref())
      .await?,
  )
}

async fn log_q(uid: u64, q: &str, li: &[Vec<u8>]) -> Result<()> {
  let q = xxai::str::low_short(q);
  let (qid, new) = qid(&q).await?;
  if new {
    trt::spawn!({
      gt::QE(format!("INSERT INTO q (id,q) VALUES ({qid},$1)"), &[&q]).await?;
    });
  }
  Ok(())
}

pub async fn log(uid: u64, level: u8, buf: &[u8], all_ws: AllWs) -> Result<()> {
  dbg!(level, &buf);
  let (_, log_li) = LogLi::unpack(&buf)?;

  for li in log_li.li {
    let li = li.li;
    if li.len() > 1 {
      let q = &li[0];
      let q = if q.is_empty() {
        "".into()
      } else {
        String::from_utf8_lossy(q)
      };
      let li = &li[1..];
      log_q(uid, &q, li).await?;
    }
  }

  Ok(())
}
