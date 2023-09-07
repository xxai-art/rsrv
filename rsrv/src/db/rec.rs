use std::collections::HashSet;

use anyhow::Result;
use clip_qdrant::qdrant_client;
use qdrant_client::qdrant::{
  point_id::PointIdOptions, BatchResult, Condition, Filter, PointId, RecommendBatchPoints,
  RecommendPoints,
};

use crate::{
  db::score,
  C,
  C::{cid::CID_IMG, 分级},
};

pub static SFW: &str = "sfw";

// 为了防止攻击，一次最多32个推荐序列, 每个序列最多32个点
const MAX: usize = 32;
const LIMIT: usize = 1024;
const TOP_K: usize = 128;

fn to_points(iter: impl Iterator<Item = u64>) -> Vec<PointId> {
  iter.map(PointId::from).collect()
}

async fn batch_sort(batch: &[BatchResult]) -> Result<Vec<Vec<u64>>> {
  let len = batch.len();
  let mut li = Vec::with_capacity(len);
  let mut exist = HashSet::new();

  for i in batch.iter() {
    let result = &i.result;
    let len = result.len();
    let mut ili = Vec::with_capacity(len);
    let mut sli = Vec::with_capacity(len);
    let mut qli = Vec::with_capacity(len);
    for j in result {
      if let Some(id) = &j.id && let Some(quality) = &j.payload.get("q") && let Some(PointIdOptions::Num(id)) = id.point_id_options && let Some(quality) = quality.as_integer() {
                ili.push(id);
                sli.push(j.score);
                qli.push(quality as _);
            }
    }

    let mut t = Vec::with_capacity(TOP_K);
    for i in score::sort(ili, sli, qli) {
      if !exist.contains(&i) {
        t.push(i);
        exist.insert(i);
        if t.len() >= TOP_K {
          break;
        }
      }
    }
    let mut r = Vec::with_capacity(t.len() + 3);
    r.push(CID_IMG.into());
    r.push(t.len() as u64);
    r.append(&mut t);
    li.push(r);
  }

  Ok(li)
}

pub async fn rec_by_action(
  level: u8,                      // 内容分级
  action_li: Vec<Vec<(u8, u64)>>, // cid, rid
) -> Result<Vec<Vec<u64>>> {
  if action_li.is_empty() {
    return Ok(vec![]);
  }

  let filter = if level == 分级::不限 {
    None
  } else {
    Some(Filter::must([if level == 分级::成人 {
      Condition::matches(SFW, false)
    } else {
      Condition::is_empty(SFW)
    }]))
  };

  let collection_name = C::CLIP.to_string();

  let recommend_points: Vec<_> = action_li
    .into_iter()
    .take(MAX)
    .map(|li| {
      let li: Vec<_> = li
        .into_iter()
        .take(MAX)
        .filter(|(cid, _)| *cid == CID_IMG)
        .map(|(_, rid)| rid)
        .collect();
      RecommendPoints {
        collection_name: collection_name.clone(),
        positive: to_points(li.into_iter()),
        negative: vec![],
        filter: filter.clone(),
        limit: LIMIT as u64,
        with_payload: Some(true.into()),
        ..Default::default()
      }
    })
    .collect();

  match qdrant_client()
    .recommend_batch(&RecommendBatchPoints {
      collection_name,
      recommend_points,
      ..Default::default()
    })
    .await
  {
    Ok(r) => batch_sort(&r.result).await,
    Err(err) => {
      tracing::error!("{:?}", err);
      Ok(vec![])
    }
  }
}
