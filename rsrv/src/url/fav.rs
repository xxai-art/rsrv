use client::Client;
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
  let user_id = client.logined().await?;
  dbg!(user_id);
  // dbg!(client.user_id().await?);
  // dbg!(client);
  //输出0-1

  Ok(1)
}
