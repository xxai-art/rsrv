use anyhow::Result;
use anypack::{Pack, VecAny};
use tokio::sync::mpsc::Sender;

use crate::{AllWs, C::SEND};

const LIMIT: usize = 4096;

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

pub async fn sync(sender: Sender<(SEND, Vec<u8>)>, uid: u64, mut pre_id: u64) -> Result<()> {
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
    sender.send((SEND::浏览, r.pack())).await?;
    pre_id = last_ts;
    if len < LIMIT {
      break;
    }
  }
  Ok(())
}
