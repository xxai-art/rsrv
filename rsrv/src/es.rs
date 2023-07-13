use anyhow::Result;
use lazy_static::lazy_static;
use x0::{fred::interfaces::SortedSetsInterface, KV};
use xxai::{b64, u64_bin};

use crate::K;

lazy_static! {
  static ref NCHAN_URL: String = std::env::var("NCHAN").unwrap();
}

pub const KIND_SYNC_FAV: u16 = 1;

pub fn publish_b64(client_id: impl AsRef<str>, kind: u16, msg: impl Into<String>) {
  let client_id = client_id.as_ref();
  let msg = msg.into();
  let nchan_url = format!("{}{client_id}", &*NCHAN_URL);
  trt::spawn!({
    reqwest::Client::new()
      .post(&nchan_url)
      .body(format!("[{kind},{msg}]"))
      .send()
      .await?;
  });
}

// pub fn publish(client_id: u64, kind: u16, msg: impl Into<String>) {
//   let client_id = b64(u64_bin(client_id));
//   publish_b64(client_id, kind, msg);
// }

pub fn publish_to_user_client(sender_client_id: u64, uid: u64, kind: u16, msg: impl Into<String>) {
  let msg = msg.into();
  trt::spawn!({
    let msg = format!("{uid},{}", msg);
    let sender_client_id = &u64_bin(sender_client_id)[..];

    for client_id in client_id_by_uid(uid).await? {
      if &client_id[..] != sender_client_id {
        let client_id = b64(client_id);
        publish_b64(client_id, kind, &msg);
      }
    }
  });
}

const TIMEOUT: u64 = 100;

pub async fn client_id_by_uid(uid: u64) -> Result<Vec<Vec<u8>>> {
  let key = &*K::nchan(uid);
  let now = xxai::now();
  let p = KV.pipeline();
  p.zremrangebyscore(key, "-inf", (now - TIMEOUT) as f64)
    .await?;
  p.zrange(key, 0, -1, None, false, None, false).await?;
  Ok(p.last().await?)
}
