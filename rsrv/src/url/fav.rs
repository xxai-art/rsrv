use axum::body::Bytes;
use client::Client;
use serde::{Deserialize, Serialize};
use x0::{fred::interfaces::HashesInterface, KV};
use xxpg::Q01;

use crate::{
  es,
  es::{publish_to_user_client, KIND_SYNC_FAV},
  K,
};

#[derive(Serialize, Debug, Deserialize)]
struct FavSync(u64, Vec<(u16, u64, u64, i8)>);

Q01!(
    fav_user:
    INSERT INTO fav.user (user_id,cid,rid,ctime,action) VALUES ($1,$2,$3,$4,$5) ON CONFLICT (user_id, cid, rid, ctime) DO NOTHING RETURNING id
);

pub async fn post(client: Client, body: Bytes) -> awp::any!() {
  let FavSync(user_id, fav_li) =
    serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;

  let mut id = 0;
  let mut n = 0;
  let mut json = String::new();
  if client.is_login(user_id).await? {
    for (cid, rid, ctime, action) in fav_li {
      if let Some(_id) = fav_user(&user_id, &cid, &rid, &ctime, &action).await? {
        id = _id;
        n += 1;
        json += &format!("{cid},{rid},{ctime},{action}");
      }
    }
  }
  if n > 0 {
    let p = KV.pipeline();
    p.hincrby(K::FAV_SUM, user_id, n).await?;
    p.hset(K::FAV_ID, (user_id, id)).await?;
    p.all().await?;

    publish_to_user_client(client.id, user_id, KIND_SYNC_FAV, format!("{json},{id}"));
  }
  Ok(id)
}
