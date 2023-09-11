mod sync;

use anyhow::Result;

use crate::{
  db::{fav_insert, seen_insert},
  r#type::AllWs,
  C::RECV,
};

pub async fn recv(action: RECV, msg: &[u8], uid: u64, client_id: u64, all_ws: AllWs) -> Result<()> {
  dbg!(&action);
  match action {
    RECV::服务器传浏览器 => {
      sync::sync(msg, uid, client_id, all_ws).await?;
    }
    RECV::浏览器传服务器 => {
      let mut msg = vb::d(msg)?;
      let len = msg.len();
      if len > 1 {
        let table = msg.pop().unwrap();
        let prev_id = msg.pop().unwrap();

        if table == 0 {
          // fav
          fav_insert::insert(uid, prev_id, &msg).await?;
        } else if table == 1 {
          //seen
          seen_insert::insert(uid, prev_id, &msg).await?;
        }
      }
    }
  }
  Ok(())
}
