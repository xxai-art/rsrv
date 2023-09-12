use anyhow::Result;

use crate::r#type::AllWs;

pub async fn log(level: u8, msg: &[u8], all_ws: AllWs) -> Result<()> {
  dbg!(level, &msg);
  Ok(())
}
