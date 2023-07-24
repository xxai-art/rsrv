use anyhow::Result;
use x0::{fred::interfaces::HashesInterface, KV};
use xxai::{bin_u64, u64_bin};

pub fn set_last(key: &'static [u8], uid: u64, id: u64) {
  trt::spawn!({
    KV.hset(key, (u64_bin(uid), u64_bin(id))).await?;
  });
}

pub async fn has_more(
  key: &'static [u8],
  uid_bin: impl AsRef<[u8]>,
  last_id: u64,
) -> Result<Option<u64>> {
  let pre_last_id: Option<Vec<u8>> = KV.hget(key, uid_bin.as_ref()).await?;

  Ok(if let Some(pre_last_id) = pre_last_id {
    let pre_last_id = bin_u64(pre_last_id);
    if pre_last_id == last_id {
      None
    } else {
      Some(pre_last_id)
    }
  } else {
    None
  })
}
