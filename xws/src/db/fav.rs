use anyhow::Result;
use anypack::{Pack, VecAny};
use tokio::sync::mpsc::Sender;
use xg::Q;

use crate::{AllWs, C::SEND};
const LIMIT: usize = 4096;

Q! {
    fav_li:SELECT id,cid,rid,ts,aid FROM fav.user WHERE uid=$1 AND id>$2 ORDER BY id LIMIT 4096;
}

pub async fn sync(sender: Sender<(SEND, Vec<u8>)>, uid: u64, mut pre_id: u64) -> Result<()> {
  while let Ok(li) = fav_li(uid, pre_id).await {
    let len = li.len();
    if len == 0 {
      break;
    }
    let mut r = VecAny::with_capacity(len * 4 + 1);
    let id = li[len - 1].0;
    for (_, cid, rid, ts, aid) in li {
      r.push(cid);
      r.push(rid);
      r.push(ts);
      r.push(aid);
    }
    r.push(pre_id);
    r.push(id);
    sender.send((SEND::收藏, r.pack())).await?;
    pre_id = id;
    if len < LIMIT {
      break;
    }
  }
  Ok(())
}
