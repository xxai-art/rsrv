use anyhow::Result;

use crate::AllWs;

pub async fn sync(msg: &[u8], uid: u64, client_id: u64, all_ws: AllWs) -> Result<()> {
  let body = vb::d(msg)?;
  let mut to_sync = [
    0, // 收藏
    0, // 浏览
  ];
  for i in body.chunks(2) {
    let p = i[0] as usize;
    if p < to_sync.len() {
      to_sync[p] = i[1];
    }
  }
  // 同步::收藏(uid, channel_id.clone(), to_sync[0]);
  // 同步::浏览(uid, channel_id, to_sync[1]);
  Ok(())
}
