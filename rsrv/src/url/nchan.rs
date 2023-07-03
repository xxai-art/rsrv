use axum::{
  extract::{FromRequest, FromRequestParts},
  handler::Handler,
  http::request::Request,
  response::{IntoResponse, Response},
};
use client::Client;

pub async fn get(mut client: Client) -> awp::Result<Response> {
  let user_id = client.user_id().await?;
  dbg!(user_id);
  // let FavSync(user_id, fav_li) =
  //   serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;
  //
  // let mut id = 0;
  // let mut n = 0;
  // if client.is_login(user_id).await? {
  //   for (cid, rid, ctime, action) in fav_li {
  //     if let Some(_id) = fav_user(&user_id, &cid, &rid, &ctime, &action).await? {
  //       id = _id;
  //       n += 1;
  //     }
  //   }
  // }
  // if n > 0 {
  //   let p = KV.pipeline();
  //   p.hincrby("favSum", user_id, n).await?;
  //   p.hset("favId", (user_id, id)).await?;
  //   p.all().await?;
  // }

  Ok("".into_response())
}
