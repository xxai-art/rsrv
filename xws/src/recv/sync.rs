use anyhow::Result;
use anypack::{Pack, Packable, VecAny};
use tokio::{
  sync::mpsc::{channel, Sender},
  time::{timeout, Duration},
};

use crate::{AllWs, C::SEND};

pub async fn sync(msg: &[u8], uid: u64, client_id: u64, all_ws: AllWs) -> Result<()> {
  let body = vb::d(msg)?;
  let mut to_sync = [
    0, // 收藏
    0, // 浏览
  ];
  for i in body.chunks(2) {
    let p = i[0] as usize;
    if p < to_sync.len() {
      to_sync[p] = i[1];
    }
  }

  let (send, mut recv) = channel(3);
  trt::spawn!({
    xerr::log!(crate::db::fav::sync(send, uid, to_sync[1]).await);
  });

  while let Some((action, bin)) = recv.recv().await {
    all_ws.to_client(uid, client_id, action, &bin).await?;
  }

  // let all = all_ws.clone();
  // trt::spawn!({
  //   xerr::log!(crate::db::seen::sync(uid, client_id, to_sync[1], all).await);
  //   sx.send(()).await?;
  // });

  // let mut n = 0;
  // loop {
  //   let _ = timeout(Duration::from_secs(3), rx.recv()).await;
  //   n += 1;
  //   if n == to_sync.len() {
  //     break;
  //   }
  // }
  // all_ws
  //   .to_client(uid, client_id, SEND::服务器传浏览器完成, &[])
  //   .await?;
  Ok(())
}
