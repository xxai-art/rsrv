use axum::body::Bytes;
use client::Client;
use serde::{Deserialize, Serialize};
use xxpg::Q;

#[derive(Serialize, Debug, Deserialize)]
struct FavSync(u64, Vec<(u16, u64, u64, i8)>);
// use crate::cs::FAV_INSERT;

// use axum::extract::Host;
// use client::client;
// use tower_cookies::Cookies;
//use x0::R;

// use anypack::url_fn;

Q!(
  fav_user:
    INSERT INTO fav.user (user_id,cid,rid,ctime,action) VALUES ($1,$2,$3,$4,$5) ON CONFLICT (user_id, cid, rid, ctime) DO NOTHING
);

pub async fn post(mut client: Client, body: Bytes) -> awp::any!() {
  let FavSync(user_id, fav_li) =
    serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;

  // let mut id_li = Vec::<Vec<u8>>::with_capacity(fav_li.len());
  if client.is_login(user_id).await? {
    //{cid: 2, rid: 215060, ctime: 1688364595987, action: 0}
    for (cid, rid, ctime, action) in fav_li {
      fav_user(&user_id, &cid, &rid, &ctime, &action).await?;
      //   // id_li.push(vbyte::compress_list(&[cid as u64, rid]));
    }
    // fav_user(
    //   fav_li
    //     .into_iter()
    //     .map(|(cid, rid, ctime, action)| format!("({user_id},{cid},{rid},{ctime},{action})",))
    //     .collect(),
    // )
    // .await?;
  }
  // let user_id: u64 = body[0].into();

  // dbg!(client.user_id().await?);
  // dbg!(client);
  //输出0-1
  // CS.sql("INSERT INTO fav (ts,ctime,uid,action,kind,rid) VALUES ({},{},{},{},{},{})");
  //FAV_INSERT.exe().await;
  // let mut ts = coarsetime::Clock::now_since_epoch().as_millis();
  // let mut ctime = ts;
  //
  // dbg!(body);
  // FAV_INSERT.exe(
  //   ts,
  //   ctime,
  //   uid,
  //   FAV_ACTION_NEW,
  //   FAV_ACTION_IMG,
  //   rid
  // ).await;
  Ok(1)
}
