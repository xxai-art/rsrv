use std::collections::HashSet;

use anyhow::Result;

pub async fn insert(prev_id: u64, li: &[u64]) -> Result<()> {
  // let mut to_insert = Vec::new();
  // let mut to_publish = Vec::new();
  let mut ts = sts::ms();

  // let cid = cid_rid_li[0];
  // let mut rid_set = HashSet::with_capacity(cid_rid_li.len() - 1);
  //
  // let mut pre = 0;
  // for i in &cid_rid_li[1..] {
  //   pre += i;
  //   rid_set.insert(pre);
  // }
  //
  // if !rid_set.is_empty() {
  //   let rid_in = rid_set
  //     .iter()
  //     .map(|x| x.to_string())
  //     .collect::<Vec<String>>()
  //     .join(",");
  //
  //   for i in gt::Q(
  //     format!("SELECT rid FROM seen WHERE uid={uid} AND cid={cid} AND rid IN ({rid_in})"),
  //     &[],
  //   )
  //   .await?
  //   {
  //     let rid: i64 = i.get(0);
  //     rid_set.remove(&(rid as u64));
  //   }
  // }
  // if !rid_set.is_empty() {
  //   let mut publish = Vec::with_capacity(rid_set.len() + 1);
  //   for rid in rid_set {
  //     publish.push(rid);
  //     to_insert.push(format!("({uid},{cid},{rid},{ts})"));
  //     ts += 1;
  //   }
  //   xxai::diffli(&mut publish);
  //
  //   publish.push(cid);
  //   let publish = xxai::z85_encode_u64_li(publish);
  //   to_publish.push(format!("\"{publish}\""));
  // }
  Ok(())
}
