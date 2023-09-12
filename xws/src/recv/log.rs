use anyhow::Result;
use serde::Deserialize;

use crate::r#type::AllWs;

#[derive(Debug, Deserialize, PartialEq)]
struct Log(Vec<Vec<Vec<u8>>>);

pub async fn log(level: u8, msg: &[u8], all_ws: AllWs) -> Result<()> {
  dbg!(level, &msg);
  let log: Log = rmp_serde::from_slice(&msg)?;
  dbg!(&log);
  Ok(())
}
