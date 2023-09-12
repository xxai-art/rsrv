use anyhow::Result;
use msgpacker::prelude::*;

use crate::r#type::AllWs;

#[derive(Debug, PartialEq, Eq, MsgPacker)]
struct Log {
  li: Vec<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq, MsgPacker)]
struct LogLi {
  li: Vec<Log>,
}

pub async fn log(level: u8, buf: &[u8], all_ws: AllWs) -> Result<()> {
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
      dbg!(q, li);
    }
  }

  Ok(())
}
