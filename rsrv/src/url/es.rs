use axum::{
  extract::Path,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use client::Client;
use x0::{fred::interfaces::SortedSetsInterface, KV};
use xxai::u64_bin;

use crate::es;

pub fn es_sync(channel_id: String, li: Box<[u64]>) {
  tokio::spawn(async move {
    let fav_synced = li[0];
    let fav_synced_id = li[1];
    dbg!(fav_synced, fav_synced_id);
    // TODO KV.hmget
    es::publish_b64(channel_id, "good s");
  });
}

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

      es_sync(channel_id, Box::from(&li[1..]));

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
