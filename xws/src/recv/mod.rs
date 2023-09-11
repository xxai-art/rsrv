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
      let msg = vb::d(msg)?;
      let len = msg.len();
      if len > 0 {
        let len = len - 1;
        let table = msg[len] as usize;
        let msg = &msg[0..len];
        dbg!("浏览器传服务器", msg.len(), table, uid, client_id);
      }
    }
  }
  Ok(())
}
