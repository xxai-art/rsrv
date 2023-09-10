use anyhow::Result;

use crate::C::RECV;

pub async fn recv(action: RECV, bin: &[u8]) -> Result<Option<Box<[u8]>>> {
  // match action {}
  Ok(None)
}
