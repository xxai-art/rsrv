use anyhow::Result;

use crate::{r#type::AllWs, C::RECV};

pub async fn recv(
  action: RECV,
  bin: &[u8],
  user_ws: impl Fn() -> AllWs,
) -> Result<Option<Box<[u8]>>> {
  match action {
    RECV::同步 => {}
  }
  Ok(None)
}
