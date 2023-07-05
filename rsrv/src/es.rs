use lazy_static::lazy_static;
use x0::KV;
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
  tokio::spawn(async move {
    reqwest::Client::new()
      .post(&nchan_url)
      .body(format!("[{kind},{msg}]"))
      .send()
      .await?;
    Ok::<(), anyhow::Error>(())
  });
}

// pub fn publish(client_id: u64, kind: u16, msg: impl Into<String>) {
//   let client_id = b64(u64_bin(client_id));
//   publish_b64(client_id, kind, msg);
// }

pub fn publish_to_user_client(
  sender_client_id: u64,
  user_id: u64,
  kind: u16,
  msg: impl Into<String>,
) {
  let msg = msg.into();
  tokio::spawn(async move {
    let msg = format!("{user_id},{}", msg);
    let sender_client_id = &u64_bin(sender_client_id)[..];

    for client_id in client_id_by_user_id(user_id).await {
      if &client_id[..] != sender_client_id {
        let client_id = b64(client_id);
        publish_b64(client_id, kind, &msg);
      }
    }
    Ok::<(), anyhow::Error>(())
  });
}

pub async fn client_id_by_user_id(user_id: u64) -> Vec<Vec<u8>> {
  let key = &*K::nchan(user_id);
  let r = vec![];
  let now = xxai::now();
  let p = KV.pipeline();
  // p.hincrby(K::FAV_SUM, user_id, n).await?;
  // p.hset(K::FAV_ID, (user_id, id)).await?;
  // p.all().await?;
  r
}
