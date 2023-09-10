use anyhow::Result;

use crate::{r#type::AllWs, C::RECV};

pub async fn recv(
  action: RECV,
  bin: &[u8],
  uid: u64,
  client_id: u64,
  all_ws: AllWs,
) -> Result<Option<Box<[u8]>>> {
  match action {
    RECV::同步 => {
      dbg!(uid, client_id);
    }
  }
  Ok(None)
}
