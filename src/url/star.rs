use anypack::sync_url_fn;
use axum::extract::Host;
use tower_cookies::Cookies;
use trt::TRT;
use xkv::fred::interfaces::HashesInterface;
use xxhash_rust::xxh3::xxh3_64;

use crate::R;

// use anypack::url_fn;
// use xxpg::Q;
//
// Q!(
//     img_li:
//     SELECT task.id,hash::bytea,w,h,star,laugh FROM bot.task,bot.civitai_img WHERE hash IS NOT NULL AND bot.task.rid=bot.civitai_img.id AND task.adult=0 AND cid=1 ORDER BY star DESC LIMIT 600
// );

const TOKEN_LEN: usize = 8;

static mut SK: [u8; 32] = [0; 32];

#[ctor::ctor]
fn init() {
  TRT.block_on(async move {
    let sk: Vec<u8> = R.force().await.hget("conf", "SK").await.unwrap();
    unsafe { SK = sk.try_into().unwrap() };
  })
}

pub fn tld(host: impl AsRef<str>) -> String {
  let host = host.as_ref();
  if let Some(p) = host.find(':') {
    &host[..p]
  } else {
    &host
  }
  .to_string()
}

pub fn cookie_client_id(Host(host): Host, cookies: &Cookies) -> Option<u64> {
  use tower_cookies::{
    cookie::{time::Duration, SameSite},
    Cookie,
  };

  dbg!(tld(&host));

  cookies.add(
    Cookie::build("hello_world_key", "hello_world_val4")
      .max_age(Duration::seconds(99999999))
      .secure(true)
      .path("/")
      .domain(tld(&host))
      .same_site(SameSite::None)
      .http_only(true)
      .finish(),
  );
  if let Some(c) = cookies.get("I") {
    if let Ok(c) = xxai::cookie_decode(c.value()) {
      if c.len() >= TOKEN_LEN {
        let client = &c[TOKEN_LEN..];
        if xxh3_64(&[unsafe { &SK }, client].concat())
          == u64::from_le_bytes(c[..TOKEN_LEN].try_into().unwrap())
        {
          dbg!(client);
        }
      }
    }
  }
  None
}

sync_url_fn!(post(host: Host, cookies: Cookies) {
    cookie_client_id(host, &cookies);

    1
});
