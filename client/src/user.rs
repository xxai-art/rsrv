use std::{error::Error, fmt::Display};

use axum::http::StatusCode;
use axum_derive_error::ErrorResponse;
use x0::{fred::interfaces::FunctionInterface, R};
use xxai::bin_u64;

const R_CLIENT_USER: &[u8] = &[4, 0];
const ZMAX: &str = "zmax";

#[derive(ErrorResponse, PartialEq, Eq)]
pub enum AuthErr {
  #[status(StatusCode::UNPROCESSABLE_ENTITY)]
  NoLogin,
}

impl Display for AuthErr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}

impl Error for AuthErr {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

impl crate::Client {
  pub async fn logined(&mut self) -> anyhow::Result<u64> {
    if let Some(id) = self.user_id().await? {
      return Ok(id);
    }
    Err(AuthErr::NoLogin.into())
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
