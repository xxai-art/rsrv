mod sync;

use anyhow::Result;

use crate::{r#type::AllWs, C::RECV};

pub async fn recv(action: RECV, msg: &[u8], uid: u64, client_id: u64, all_ws: AllWs) -> Result<()> {
  match action {
    RECV::同步 => {
      sync::sync(msg, uid, client_id, all_ws).await?;
    }
  }
  Ok(())
}
