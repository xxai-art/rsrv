use anyhow::Result;
use x0::{fred::interfaces::SortedSetsInterface, KV};

use crate::K;

// use crate::K;

// use xg::Q;
// Q!(
// li:
// SELECT task.id FROM bot.task,bot.civitai_img WHERE hash IS NOT NULL AND bot.task.rid=bot.civitai_img.id AND task.adult=0 AND cid=1 ORDER BY star DESC LIMIT 512
// );

pub async fn rec(z85: &Vec<u64>) -> Result<Vec<u64>> {
  let level = z85[0];
  let offset = z85[1];
  let key = if level == 2 {
    // 成人
    K::REC1
  } else if level == 1 {
    // 不限
    K::REC
  } else {
    // 安全
    K::REC0
  };
  Ok(li(key, offset).await?)
}

pub async fn li(key: &[u8], offset: u64) -> Result<Vec<u64>> {
  let offset = offset as i64;
  let bin_li: Vec<Vec<u8>> = KV.zrevrange(key, offset, offset + 128, false).await?;
  let li: Vec<u64> = bin_li
    .iter()
    .map(|i| match vb::d(i) {
      Ok(i) => Ok([i[0] as u64, i[1]]),
      Err(err) => Err(err),
    })
    .filter_map(Result::ok)
    .flat_map(|v| v)
    .collect();
  // let li = vec![];
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
