mod sync;

use anyhow::Result;

use crate::{r#type::AllWs, C::RECV};

pub async fn recv(action: RECV, msg: &[u8], uid: u64, client_id: u64, all_ws: AllWs) -> Result<()> {
  dbg!(&action);
  match action {
    RECV::服务器传浏览器 => {
      sync::sync(msg, uid, client_id, all_ws).await?;
    }
    RECV::浏览器传服务器 => {
      dbg!("浏览器传服务器", msg, uid, client_id);
    }
  }
  Ok(())
}
