use axum::body::Bytes;
use client::Client;
use serde::{Deserialize, Serialize};
use x0::{fred::interfaces::HashesInterface, KV};
use xxpg::Q;

use crate::{
  es::{publish_to_user_client, KIND_SYNC_FAV},
  K,
};

#[derive(Serialize, Debug, Deserialize)]
struct Data(u64, Vec<Vec<u64>>);

Q!(
  fav_ym:
    SELECT cid,rid,ctime,action FROM fav.user WHERE user_id=$1 AND ctime>=$2 AND ctime<=$3
);

pub async fn post(client: Client, body: Bytes) -> awp::any!() {
  let mut li = Vec::new();
  let Data(user_id, ym_li_li) =
    serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;
  // let mut id = 0;
  // let mut n = 0;
  // let mut json = String::new();
  if client.is_login(user_id).await? {
    for ym_li in ym_li_li {
      let ym = *&ym_li[0];
      let fav_li = &ym_li[1..];
      dbg!(xxai::n_to_year_month(ym as u32));
    }
  }
  //   // batch_insert!(
  //   //   "INSERT INTO fav.user (user_id,cid,rid,ctime,action) VALUES {} ON CONFLICT (user_id, cid, rid, ctime) DO NOTHING RETURNING id",
  //   //   fav_li.into_iter().map(|x|( user_id,x.0,x.1,x.2,x.3 )).collect::<Vec<_>>()
  //   // );
  //   for (cid, rid, ctime, action) in fav_li {
  //     if let Some(_id) = fav_user(&user_id, &cid, &rid, &ctime, &action).await? {
  //       id = _id;
  //       n += 1;
  //       json += &format!("{cid},{rid},{ctime},{action},");
  //     }
  //   }
  // }
  // if n > 0 {
  //   let p = KV.pipeline();
  //   p.hincrby(K::FAV_SUM, user_id, n).await?;
  //   p.hset(K::FAV_ID, (user_id, id)).await?;
  //   p.all().await?;
  //
  //   publish_to_user_client(client.id, user_id, KIND_SYNC_FAV, format!("{json}{id}"));
  // }
  Ok(li)
}
