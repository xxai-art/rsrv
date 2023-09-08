use anyhow::Result;

use crate::C::WR;

pub async fn ws(action: WR, uid: u64, channel_id: String, body: &[u8]) -> Result<()> {
  Ok(())
}
