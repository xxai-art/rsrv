use std::{error::Error, fmt::Display};

use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use http_body::combinators::UnsyncBoxBody;
use thiserror::Error;
use x0::{fred::interfaces::FunctionInterface, R};
use xxai::bin_u64;

const R_CLIENT_USER: &[u8] = &[4, 0];
const ZMAX: &str = "zmax";

#[derive(Debug, Error)]
pub enum AuthErr {
  #[error("NoLogin")]
  NoLogin,
  #[error("{0:?}")]
  Err(anyhow::Error),
}

impl From<anyhow::Error> for AuthErr {
  fn from(err: anyhow::Error) -> Self {
    Self::Err(err)
  }
}

impl IntoResponse for AuthErr {
  fn into_response(self) -> Response<UnsyncBoxBody<axum::body::Bytes, axum::Error>> {
    match self {
      AuthErr::NoLogin => (StatusCode::PRECONDITION_FAILED, "NoLogin".to_string()), // 412
      AuthErr::Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:?}")),     // 500
    }
    .into_response()
  }
}

impl crate::Client {
  pub async fn logined(&mut self) -> std::result::Result<u64, AuthErr> {
    if let Some(id) = self.user_id().await? {
      return Ok(id);
    }
    Err(AuthErr::NoLogin)
  }

  pub async fn user_id(&mut self) -> anyhow::Result<Option<u64>> {
    Ok(if let Some(id) = self._user_id {
      if id == 0 {
        let key = [R_CLIENT_USER, &xxai::u64_bin(self.id)].concat();
        // let id: Option<u64> = R.fcall_ro(ZMAX, vec![&key[..]], vec![0]).await?;
        let id: Option<Vec<u8>> = R.fcall_ro(ZMAX, vec![&key[..]], vec![0]).await.unwrap();
        let id = if let Some(id) = id {
          Some(bin_u64(id))
        } else {
          None
        };
        self._user_id = id;
        id
      } else {
        Some(id)
      }
    } else {
      None
    })
  }
}
