use anyhow::Result;
use anypack::{Any, Pack, VecAny};
use intbin::u64_bin;
use lazy_static::lazy_static;
use reqwest::header;
use ub64::b64e;
use x0::{fred::interfaces::SortedSetsInterface, KV};

use crate::{C::WS, K};

lazy_static! {
  static ref NCHAN_URL: String = std::env::var("NCHAN").unwrap();
}

pub const KIND_SYNC_FAV: u16 = 1;
pub const KIND_SYNC_SEEN: u16 = 2;

// let client_id = u64_bin(client_id);
// let channel_id = b64e(&client_id[..]);
pub async fn send(channel_id: impl AsRef<str>, kind: WS, msg: impl Into<Any>) -> Result<()> {
  let msg = msg.into();
  let channel_id = channel_id.as_ref();
  let nchan_url = format!("{}{channel_id}", &*NCHAN_URL);
  let mut li = VecAny::new();
  li.push(kind as u8);
  li.push(msg);
  dbg!(&nchan_url);
  reqwest::Client::new()
    .post(&nchan_url)
    .header(header::CONTENT_TYPE, "application/octet-stream")
    .body(li.pack())
    .send()
    .await?;
  Ok(())
}

// pub fn publish(client_id: u64, kind: u16, msg: impl Into<String>) {
//   let client_id = b64(u64_bin(client_id));
//   send(client_id, kind, msg);
// }

pub fn channel_id_by_uid_client_id(uid: u64, client_id: u64) -> String {
  let uid = ub64::b64_u64(uid);
  let client_id = vb::e(&[uid, client.id]);
  b64e(&client_id[..])
}

pub fn send_user(uid: u64, sender_client_id: u64, kind: WS, msg: impl Into<Any>) {
  let msg = msg.into();
  trt::spawn!({
    let sender_client_id = &u64_bin(sender_client_id)[..];
    for client_id in client_id_by_uid(uid).await? {
      if &client_id[..] != sender_client_id {
        let channel_id = bin_u64(channel_id);
        send(channel_id_by_uid_client_id(uid, channel_id), kind, msg).await?;
      }
    }
  });
}

const TIMEOUT: u64 = 610;

pub async fn client_id_by_uid(uid: u64) -> Result<Vec<Vec<u8>>> {
  let key = &*K::nchan(uid);
  let now = xxai::now();
  let p = KV.pipeline();
  p.zremrangebyscore(key, "-inf", (now - TIMEOUT) as f64)
    .await?;
  p.zrange(key, 0, -1, None, false, None, false).await?;
  Ok(p.last().await?)
}
