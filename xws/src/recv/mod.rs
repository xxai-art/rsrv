mod sync;

use anyhow::Result;

use crate::{r#type::AllWs, C::RECV};

pub async fn recv(action: RECV, msg: &[u8], uid: u64, client_id: u64, all_ws: AllWs) -> Result<()> {
  match action {
    RECV::浏览器同步服务器 => {
      sync::sync(msg, uid, client_id, all_ws).await?;
    }
    RECV::服务器同步浏览器 => {
      dbg!("服务器同步浏览器", msg, uid, client_id);
    }
  }
  Ok(())
}
