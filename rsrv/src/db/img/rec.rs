use anyhow::Result;
use intbin::bin_u64;
use x0::{fred::interfaces::SortedSetsInterface, KV};

// use xg::Q;
use crate::{cid::CID_IMG, K};

// Q!(
// li:
// SELECT task.id FROM bot.task,bot.civitai_img WHERE hash IS NOT NULL AND bot.task.rid=bot.civitai_img.id AND task.adult=0 AND cid=1 ORDER BY star DESC LIMIT 512
// );

pub async fn li() -> Result<Vec<u64>> {
  let key = K::REC0;
  let bin_li: Vec<Vec<u8>> = KV.zrevrange(key, 0, 1000, false).await?;
  let li: Vec<u64> = bin_li.iter().map(bin_u64).collect();
  // let nsfw_li: Vec<bool> = KV
  //   .smismember(
  //     K::NSFW,
  //     bin_li
  //       .into_iter()
  //       .map(|i| {
  //         let i: RedisValue = (&i[..]).into();
  //         i
  //       })
  //       .collect::<Vec<_>>(),
  //   )
  //   .await?;
  // let mut r = Vec::with_capacity(li.len());
  // for (id, nsfw) in li.into_iter().zip(nsfw_li) {
  //   if !nsfw {
  //     r.push(id);
  //   }
  // }
  Ok(li)
}

pub async fn img_li() -> Result<Vec<u64>> {
  let mut r = Vec::new();
  for i in li().await? {
    r.push(CID_IMG);
    r.push(i);
  }
  Ok(r)
}
