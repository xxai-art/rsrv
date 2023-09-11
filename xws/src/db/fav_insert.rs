use anyhow::Result;

pub async fn insert(uid: u64, prev_id: u64, li: &[u64]) -> Result<()> {
  let len = li.len();
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
  Ok(())
}
