use lazy_static::lazy_static;

lazy_static! {
  static ref NCHAN_URL: String = std::env::var("NCHAN").unwrap();
}

pub fn publish_b64(channel_id: impl AsRef<str>, msg: impl Into<String>) {
  let channel_id = channel_id.as_ref();
  let msg = msg.into();
  let nchan_url = format!("{}{channel_id}", &*NCHAN_URL);
  tokio::spawn(async move {
    reqwest::Client::new()
      .post(&nchan_url)
      .body(msg)
      .send()
      .await?;
    Ok::<(), anyhow::Error>(())
  });
}
