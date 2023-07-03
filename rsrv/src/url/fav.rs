use axum::body::Bytes;
use client::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
struct FavSync(u64, Vec<(u16, u64, u64, u8)>);
// use crate::cs::FAV_INSERT;

// use axum::extract::Host;
// use client::client;
// use tower_cookies::Cookies;
//use x0::R;

// use anypack::url_fn;
// use xxpg::Q;
//
// Q!(
//     img_li:
//     SELECT task.id,hash::bytea,w,h,star,laugh FROM bot.task,bot.civitai_img WHERE hash IS NOT NULL AND bot.task.rid=bot.civitai_img.id AND task.adult=0 AND cid=1 ORDER BY star DESC LIMIT 600
// );

pub async fn post(mut client: Client, body: Bytes) -> awp::any!() {
  let FavSync(user_id, fav_li) =
    serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;

  let mut id_li = Vec::<Vec<u8>>::with_capacity(fav_li.len());
  if client.is_login(user_id).await? {
    //{cid: 2, rid: 215060, ctime: 1688364595987, action: 0}
    for (cid, rid, ctime, action) in fav_li {
      id_li.push(vbyte::compress_list(&[cid as u64, rid]));
    }
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
