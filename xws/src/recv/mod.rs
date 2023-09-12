mod sync;
use anyhow::Result;
use anypack::Pack;

use crate::{
  db::{fav_insert, seen_insert},
  r#type::AllWs,
  C::{RECV, SEND},
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

        macro_rules! send {
          ($send:ident, $mod:ident) => {
            all_ws
              .to_user(
                uid,
                SEND::$send,
                &$mod::insert(uid, prev_id, &msg).await?.pack(),
              )
              .await?
          };
        }

        match table {
          0 => send!(收藏, fav_insert),
          1 => send!(浏览, seen_insert),
          _ => {}
        };
      }
    }
    RECV::用户行为日志 => {
      dbg!(&msg);
    }
  }
  Ok(())
}
