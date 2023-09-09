use std::ops::{Deref, DerefMut};

use awp::Error;
use axum::{http::StatusCode, response::IntoResponse};
use xuser::ClientUser;

#[derive(Debug, Clone, Copy)]
pub struct _Client(pub ClientUser);

impl _Client {
  pub async fn logined(&mut self) -> std::result::Result<u64, awp::Err> {
    if let Some(id) = self.uid().await? {
      return Ok(id);
    }
    Err(awp::Err(Error::Response(
      (StatusCode::UNAUTHORIZED, "need login".to_string()).into_response(),
    )))
  }
}

impl Deref for _Client {
  type Target = ClientUser;
  fn deref(&self) -> &<Self as Deref>::Target {
    &self.0
  }
}

impl DerefMut for _Client {
  fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
    &mut self.0
  }
}
