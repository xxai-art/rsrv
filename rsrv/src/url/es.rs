use axum::{
  extract::Path,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use client::Client;
use lazy_static::lazy_static;
use x0::{fred::interfaces::SortedSetsInterface, KV};
use xxai::u64_bin;

lazy_static! {
  static ref NCHAN_URL: String = std::env::var("NCHAN").unwrap();
}

pub async fn get(mut client: Client, Path(li): Path<String>) -> awp::Result<Response> {
  let li = xxai::b64_u64_li(li);
  if li.len() >= 3 {
    let user_id = li[0];
    if client.is_login(user_id).await? {
      let _li = &li[1..];

      let client_id = u64_bin(client.id);

      KV.zadd(
        &[&b"nchan:"[..], &u64_bin(user_id)].concat()[..],
        None,
        None,
        false,
        false,
        (xxai::now() as f64, &client_id[..]),
      )
      .await?;

      let channel_id = xxai::b64(client_id);

      let nchan_url = format!("{}{channel_id}", &*NCHAN_URL);
      tokio::spawn(async move {
        reqwest::Client::new()
          .post(nchan_url)
          .body("test 1234")
          .send()
          .await?;
        Ok::<(), anyhow::Error>(())
      });

      return Ok(
        (
          StatusCode::OK,
          [
            (
              "X-Accel-Redirect",
              format!("/nchan/{}", channel_id).as_str(),
            ),
            ("X-Accel-Buffering", "no"),
          ],
          "",
        )
          .into_response(),
      );
    }
  }

  Ok((StatusCode::UNAUTHORIZED, "").into_response())
}
