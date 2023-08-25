use anyhow::Result;
use x0::{fred::interfaces::SortedSetsInterface, KV};

// use xg::Q;
use crate::{cid::CID_IMG, K};

// Q!(
// li:
// SELECT task.id FROM bot.task,bot.civitai_img WHERE hash IS NOT NULL AND bot.task.rid=bot.civitai_img.id AND task.adult=0 AND cid=1 ORDER BY star DESC LIMIT 512
// );

pub async fn li() -> Result<Vec<u64>> {
  Ok(KV.zrevrange(K::REC, 0, 1000, false).await?)
}

pub async fn img_li() -> Result<Vec<u64>> {
  let mut r = Vec::new();
  for i in li().await? {
    r.push(CID_IMG);
    r.push(i);
  }
  Ok(r)
}
