use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct CidRid {
  pub cid: u8,
  pub rid: u64,
}

#[derive(Debug)]
pub struct RecChina {
  pub action: u8,
  pub chain: Vec<CidRid>,
}

pub async fn rec_by_action(
  level: u64, // 内容分级
  cid_rid_action: HashMap<CidRid, RecChina>,
) -> Result<Vec<u64>> {
  if cid_rid_action.is_empty() {
    return Ok(vec![]);
  }
  dbg!("TODO rec_by_action", cid_rid_action);
  // 每个推荐流给它后续
  let rec = Vec::with_capacity(64);
  Ok(rec)
}
