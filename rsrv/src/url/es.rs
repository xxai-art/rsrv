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
  if li.len() >= 3 {
    let user_id = li[0];

    if client.is_login(user_id).await? {
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

      let url = format!("/nchan/{}", channel_id);

      let fav_synced = li[1];
      let fav_synced_id = li[2];
      tokio::spawn(async move {
        dbg!(fav_synced, fav_synced_id);
        // TODO KV.hmget
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
