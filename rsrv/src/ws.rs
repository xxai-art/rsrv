use anyhow::Result;
use anypack::{Any, Pack, VecAny};
use intbin::bin_u64;
use lazy_static::lazy_static;
use reqwest::header;
use ub64::b64e;
use x0::{fred::interfaces::SortedSetsInterface, KV};

use crate::{C::WS, K};

lazy_static! {
  static ref NCHAN_URL: String = std::env::var("NCHAN").unwrap();
}

pub async fn send(channel_id: impl AsRef<str>, kind: WS, msg: impl Into<Any>) -> Result<()> {
  let msg = msg.into();
  let channel_id = channel_id.as_ref();
  let nchan_url = format!("{}{channel_id}", &*NCHAN_URL);
  let mut li = VecAny::new();
  li.push(kind as u8);
  li.push(msg);
  reqwest::Client::new()
    .post(&nchan_url)
    .header(header::CONTENT_TYPE, "application/octet-stream")
    .body(li.pack())
    .send()
    .await?;
  Ok(())
}

pub fn send_user(uid: u64, sender_client_id: u64, kind: WS, msg: impl Into<Any>) {
  let msg = msg.into();
  trt::spawn!({
    for client_id in client_id_by_uid(uid).await? {
      let client_id = bin_u64(client_id);
      if client_id != sender_client_id {
        send(
          channel_id_by_uid_client_id(uid, client_id),
          kind.clone(),
          msg.clone(),
        )
        .await?;
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

pub fn channel_id_by_uid_client_id(uid: u64, client_id: u64) -> String {
  let client_id = vb::e(&[uid, client_id]);
  b64e(&client_id[..])
}
