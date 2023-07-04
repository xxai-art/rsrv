use axum::{
  extract::Path,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use client::Client;
use x0::{fred::interfaces::SortedSetsInterface, KV};
use xxai::u64_bin;

use crate::es;

pub async fn get(mut client: Client, Path(li): Path<String>) -> awp::Result<Response> {
  let li = xxai::b64_u64_li(li);
  if li.len() >= 4 {
    let user_id = li[0];
    let last_event_id = li[1];

    if client.is_login(user_id).await? {
      let _li = &li[2..];

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

      let mut url = format!("/nchan/{}", channel_id);

      if last_event_id > 0 {
        url = format!("{url}?last_event_id={last_event_id}:0");
      }

      tokio::spawn(async move {
        es::publish_b64(channel_id, "good s");
      });

      return Ok(
        (
          StatusCode::OK,
          [
            ("X-Accel-Redirect", url.as_str()),
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
