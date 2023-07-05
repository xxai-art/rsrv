use lazy_static::lazy_static;

lazy_static! {
  static ref NCHAN_URL: String = std::env::var("NCHAN").unwrap();
}

pub const KIND_SYNC_FAV: u16 = 1;

pub fn publish_b64(channel_id: impl AsRef<str>, kind: u16, msg: impl Into<String>) {
  let channel_id = channel_id.as_ref();
  let msg = msg.into();
  let nchan_url = format!("{}{channel_id}", &*NCHAN_URL);
  tokio::spawn(async move {
    reqwest::Client::new()
      .post(&nchan_url)
      .body(format!("[{kind},{msg}]"))
      .send()
      .await?;
    Ok::<(), anyhow::Error>(())
  });
}

pub fn publish(channel_id: u64, kind: u16, msg: impl Into<String>) {
  let channel_id = xxai::b64(xxai::u64_bin(channel_id));
  publish_b64(channel_id, kind, msg);
}
