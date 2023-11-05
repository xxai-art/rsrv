use x0::{
  fred::interfaces::{FunctionInterface, SortedSetsInterface},
  R,
};
use xxai::{ordered_bin_u64, u64_bin_ordered};

const R_CLIENT_USER: &[u8] = &[4, 0];
const ZMAX: &str = "zmax";

#[derive(Debug, Clone, Copy)]
pub struct ClientUser {
  pub id: u64,
  pub _uid: Option<u64>,
}

impl ClientUser {
  pub async fn is_login(&self, uid: u64) -> anyhow::Result<bool> {
    let key = &[R_CLIENT_USER, &xxai::u64_bin_ordered(self.id)].concat()[..];
    let r: Option<u64> = R.zscore(key, &u64_bin_ordered(uid)[..]).await?;
    Ok(if let Some(s) = r { s > 0 } else { false })
  }

  pub async fn uid(&mut self) -> anyhow::Result<Option<u64>> {
    Ok(if let Some(id) = self._uid {
      if id == 0 {
        let key = [R_CLIENT_USER, &xxai::u64_bin_ordered(self.id)].concat();
        // let id: Option<u64> = R.fcall_ro(ZMAX, vec![&key[..]], vec![0]).await?;
        let id: Option<Vec<u8>> = R.fcall_ro(ZMAX, vec![&key[..]], vec![0]).await.unwrap();
        let id = id.map(ordered_bin_u64);
        self._uid = id;
        id
      } else {
        Some(id)
      }
    } else {
      None
    })
  }
}
