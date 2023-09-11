use anyhow::Result;
use anypack::{Pack, VecAny};
use tokio::{
  sync::mpsc::{channel, Sender},
  time::{timeout, Duration},
};
use xg::Q;

use crate::{AllWs, C::SEND};

const LIMIT: usize = 4096;

Q! {
  fav_li:SELECT id,cid,rid,ts,aid FROM fav.user WHERE uid=$1 AND id>$2 ORDER BY id LIMIT 4096;
}

async fn seen_li(uid: u64, ts: u64) -> Result<Vec<(u64, i8, i64)>> {
  let sql = format!("SELECT CAST(ts as BIGINT) t,cid,rid FROM seen WHERE uid={uid} AND ts>ARROW_CAST({ts},'Timestamp(Millisecond,None)') ORDER BY ts LIMIT 4096");
  Ok(
    gt::Q(sql, &[])
      .await?
      .into_iter()
      .map(|i| (i.get::<_, i64>(0) as u64, i.get(1), i.get(2)))
      .collect(),
  )
}

pub fn 收藏(sender: Sender<()>, uid: u64, client_id: u64, mut pre_id: u64, all_ws: AllWs) {
  trt::spawn!({
    while let Ok(li) = fav_li(uid, pre_id).await {
      let len = li.len();
      if len == 0 {
        break;
      }
      let mut r = VecAny::with_capacity(len * 4 + 1);
      let id = li[len - 1].0;
      dbg!(id);
      for (_, cid, rid, ts, aid) in li {
        r.push(cid);
        r.push(rid);
        r.push(ts);
        r.push(aid);
      }
      r.push(pre_id);
      r.push(id);
      all_ws
        .to_client(uid, client_id, SEND::收藏, &r.pack())
        .await?;
      pre_id = id;
      if len < LIMIT {
        break;
      }
    }
    sender.send(()).await?;
  });
}

pub fn 浏览(sender: Sender<()>, uid: u64, client_id: u64, mut pre_id: u64, all_ws: AllWs) {
  trt::spawn!({
    while let Ok(li) = seen_li(uid, pre_id).await {
      let len = li.len();
      if len == 0 {
        break;
      }
      let mut r = VecAny::with_capacity(len * 3 + 1);
      let last_ts = li[len - 1].0;
      for (ts, cid, rid) in li {
        r.push(ts);
        r.push(cid);
        r.push(rid);
      }
      r.push(pre_id);
      all_ws
        .to_client(uid, client_id, SEND::浏览, &r.pack())
        .await?;
      pre_id = last_ts;
      if len < LIMIT {
        break;
      }
    }
    sender.send(()).await?;
  });
}

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
  let (sx, mut rx) = channel::<()>(2);
  收藏(sx.clone(), uid, client_id, to_sync[0], all_ws.clone());
  浏览(sx, uid, client_id, to_sync[1], all_ws.clone());

  let mut n = 0;
  loop {
    let _ = timeout(Duration::from_secs(3), rx.recv()).await;
    n += 1;
    if n == to_sync.len() {
      break;
    }
  }
  all_ws
    .to_client(uid, client_id, SEND::服务器传浏览器完成, &[])
    .await?;
  Ok(())
}
