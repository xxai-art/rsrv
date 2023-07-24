use x0::{fred::interfaces::HashesInterface, KV};
use xxai::u64_bin;

pub fn set_last(key: &'static [u8], uid: u64, id: u64) {
  trt::spawn!({
    KV.hset(key, (u64_bin(uid), u64_bin(id))).await?;
  });
}
