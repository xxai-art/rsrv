use anyhow::Result;
use xxpg::Q;

use crate::cid::CID_IMG;

Q!(
li:
SELECT task.id FROM bot.task,bot.civitai_img WHERE hash IS NOT NULL AND bot.task.rid=bot.civitai_img.id AND task.adult=0 AND cid=1 ORDER BY star DESC LIMIT 512
);

pub async fn img_li() -> Result<Vec<u64>> {
  let li = li().await?;
  let mut r = Vec::new();
  for i in li {
    r.push(CID_IMG);
    r.push(i);
  }
  Ok(r)
}
