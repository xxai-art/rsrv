use client::Client;

use crate::cs::FAV_INSERT;

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

pub async fn post(mut client: Client) -> awp::any!() {
  // sync_url_fn!(post(Extension(mut client):Extension<client::Client>) {
  // client(host, &cookies);
  let uid = client.logined().await?;
  // dbg!(client.user_id().await?);
  // dbg!(client);
  //输出0-1
  // CS.sql("INSERT INTO fav (ts,ctime,uid,action,kind,rid) VALUES ({},{},{},{},{},{})");
  //FAV_INSERT.exe().await;
  let mut ts = coarsetime::Clock::now_since_epoch().as_millis();
  let mut ctime = ts;

  // FAV_INSERT.exe(
  //   ts,
  //   ctime,
  //   uid,
  //   FAV_ACTION_NEW,
  //   FAV_ACTION_IMG,
  // ).await;
  Ok(1)
}
