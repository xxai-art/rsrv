use std::collections::HashSet;

use anyhow::Result;

pub async fn insert(uid: u64, prev_id: u64, li: &[u64]) -> Result<()> {
  let len = li.len();
  let mut n: usize = 0;
  while (n + 2) < len {
    let cid = li[n];
    n += 1;
    let take = li[n] as usize;
    n += 1;

    let mut rid_set = HashSet::with_capacity(take);
    for rid in (&li[n..]).into_iter().take(take) {
      rid_set.insert(rid);
    }
    if !rid_set.is_empty() {
      let rid_in = rid_set
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

      for i in gt::Q(
        format!("SELECT rid,ts FROM seen WHERE uid={uid} AND cid={cid} AND rid IN ({rid_in})"),
        &[],
      )
      .await?
      {
        let rid: i64 = i.get(0);
        let ts: i64 = i.get(1);
        rid_set.remove(&(rid as u64));
      }
      if !rid_set.is_empty() {
        let mut to_insert = Vec::new();
        let mut ts = sts::ms();
        for rid in rid_set {
          to_insert.push(format!("({uid},{cid},{rid},{ts})"));
          ts += 1;
        }
        let to_insert = to_insert.join(",");
        gt::QE(
          format!("INSERT INTO seen (uid,cid,rid,ts) VALUES {to_insert}"),
          &[],
        )
        .await?;
      }
    }

    n += take;
  }

  Ok(())
}
