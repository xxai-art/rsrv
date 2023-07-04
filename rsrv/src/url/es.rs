use axum::{
  extract::Path,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use client::Client;

pub async fn get(mut client: Client, Path(li): Path<String>) -> awp::Result<Response> {
  let li = xxai::b64_u64_li(li);
  if li.len() >= 3 {
    let user_id = li[0];
    if client.is_login(user_id).await? {
      let li = &li[1..];
      dbg!(client.id, li);
    }
  }

  Ok((StatusCode::UNAUTHORIZED, "").into_response())
}
