use axum::{
  extract::Path,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use client::Client;
use x0::{fred::interfaces::SortedSetsInterface, KV};
use xxai::u64_bin;

pub async fn get(mut client: Client, Path(li): Path<String>) -> awp::Result<Response> {
  let li = xxai::b64_u64_li(li);
  if li.len() >= 3 {
    let user_id = li[0];
    if client.is_login(user_id).await? {
      let client_id = client.id;
      let li = &li[1..];

      KV.zadd(
        &[&b"nchan:"[..], &u64_bin(user_id)].concat()[..],
        None,
        None,
        false,
        false,
        (xxai::now() as f64, u64_bin(client_id)),
      )
      .await?;

      return Ok(
        (
          StatusCode::OK,
          [
            (
              "X-Accel-Redirect",
              format!("/nchan/{}", xxai::u64_b64(client_id)).as_str(),
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
