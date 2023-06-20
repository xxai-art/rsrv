use x0::{fred::interfaces::FunctionInterface, R};
use xxai::bin_u64;

const R_CLIENT_USER: &[u8] = &[4, 0];
const ZMAX: &str = "zmax";

impl crate::Client {
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
