use std::collections::HashMap;

use axum::body::Bytes;
use client::Client;
use serde::{Deserialize, Serialize};
use xxpg::Q;

use crate::url::fav::fav_batch_add;

#[derive(Serialize, Debug, Deserialize)]
struct Data(u64, Vec<Vec<u64>>);

Q!(
    fav_ym:
    SELECT cid,rid,ctime,action FROM fav.user WHERE user_id=$1 AND ctime>=$2 AND ctime<=$3
);

pub async fn post(client: Client, body: Bytes) -> awp::any!() {
  let mut result = Vec::new();
  let Data(user_id, ym_li_li) =
    serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;

  if client.is_login(user_id).await? {
    let mut map = HashMap::new();
    for ym_li in ym_li_li {
      let ym = ym_li[0];
      for i in ym_li[1..].chunks(4) {
        map.insert((i[0] as u16, i[1], i[2]), i[3] as i8);
        // dbg!(cid, rid, ctime, action);
      }

      let ym = xxai::time::n_to_year_month(ym as _);
      let ms = xxai::time::ym_ms_range(ym.0, ym.1);
      for i in fav_ym(user_id, ms.0, ms.1).await? {
        let key = (i.0, i.1, i.2);
        if map.remove(&key).is_none() {
          result.push(i.0 as u64);
          result.push(i.1);
          result.push(i.2);
          result.push(i.3 as _);
        }
      }
    }

    fav_batch_add(
      client.id,
      user_id,
      map.into_iter().map(|(k, v)| (k.0, k.1, k.2, v)).collect(),
    )
    .await?;
  }
  Ok(result)
}
