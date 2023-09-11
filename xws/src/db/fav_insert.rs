use anyhow::Result;
use anypack::VecAny;

pub async fn insert(uid: u64, prev_id: u64, li: &[u64]) -> Result<VecAny> {
  let len = li.len();
  let publish = VecAny::new();
  let mut n: usize = 0;
  while (n + 2) < len {
    let cid = li[n];
    n += 1;
    let take = li[n] as usize;
    n += 1;
    for ra in (&li[n..]).chunks(2).take(take) {
      let rid = ra[0];
      let aid = ra[1];
      dbg!((cid, rid, aid));
    }
    n += 2 * take;
  }
  Ok(publish)
}
