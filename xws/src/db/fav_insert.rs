use anyhow::Result;
use anypack::VecAny;
use xg::{Q, Q01};

use crate::db::fav::fav_li;

Q01!(
    fav_insert:
    INSERT INTO fav.user (uid,cid,rid,ts,aid) VALUES ($1,$2,$3,$4,$5) ON CONFLICT (uid,cid,rid,ts) DO NOTHING RETURNING id;
);

Q!(
    fav_rm:
    DELETE FROM fav.user WHERE uid=$1 AND cid=$2 AND rid=$3;
);

pub async fn insert(uid: u64, prev_id: u64, li: &[u64]) -> Result<VecAny> {
  let len = li.len();
  let mut publish = VecAny::new();
  let mut last_id = prev_id;

  while let Ok(sli) = fav_li(uid, last_id).await {
    let len = sli.len();
    if len == 0 {
      break;
    }
    last_id = sli[len - 1].0;

    for (_, cid, rid, ts, aid) in sli {
      publish.push(cid);
      publish.push(rid);
      publish.push(ts);
      publish.push(aid);
    }
  }

  let mut n: usize = 0;
  let mut ts = 0;
  while (n + 2) < len {
    let cid = li[n];
    n += 1;
    let take = li[n] as usize;
    n += 1;
    if take > 0 {
      let cid = cid as u16;
      if ts == 0 {
        ts = sts::ms();
      }
      for ra in (&li[n..]).chunks(2).take(take) {
        let rid = ra[0];
        let aid = ra[1] as i8;
        fav_rm(uid, cid, rid).await?;
        if let Some(id) = fav_insert(uid, cid, rid, ts, aid).await? {
          last_id = id;
          publish.push(cid);
          publish.push(rid);
          publish.push(ts);
          publish.push(aid);
        }
        ts += 1;
      }
      n += 2 * take;
    }
  }
  publish.push(prev_id);
  publish.push(last_id);
  Ok(publish)
}
