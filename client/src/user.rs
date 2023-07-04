use awp::Error;
use axum::{http::StatusCode, response::IntoResponse};
use x0::{
  fred::interfaces::{FunctionInterface, SortedSetsInterface},
  R,
};
use xxai::{ordered_bin_u64, u64_bin_ordered};

const R_CLIENT_USER: &[u8] = &[4, 0];
const ZMAX: &str = "zmax";

impl crate::_Client {
  pub async fn logined(&mut self) -> std::result::Result<u64, awp::Err> {
    if let Some(id) = self.user_id().await? {
      return Ok(id);
    }
    Err(awp::Err(Error::Response(
      (StatusCode::UNAUTHORIZED, "need login".to_string()).into_response(),
    )))
  }

  pub async fn is_login(&mut self, user_id: u64) -> anyhow::Result<bool> {
    let key = &[R_CLIENT_USER, &xxai::u64_bin_ordered(self.id)].concat()[..];
    let r: Option<u64> = R.zscore(key, &u64_bin_ordered(user_id)[..]).await?;
    Ok(if let Some(s) = r { s > 0 } else { false })
  }

  pub async fn user_id(&mut self) -> anyhow::Result<Option<u64>> {
    Ok(if let Some(id) = self._user_id {
      if id == 0 {
        let key = [R_CLIENT_USER, &xxai::u64_bin_ordered(self.id)].concat();
        // let id: Option<u64> = R.fcall_ro(ZMAX, vec![&key[..]], vec![0]).await?;
        let id: Option<Vec<u8>> = R.fcall_ro(ZMAX, vec![&key[..]], vec![0]).await.unwrap();
        let id = id.map(ordered_bin_u64);
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
