#![feature(lazy_cell)]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![feature(let_chains)]
#![allow(non_snake_case)]

use awp::anypack::FnAny;
use axum::{
  middleware,
  routing::{get, post},
  Router,
};
use client::client;
use trt::TRT;

mod es;
mod url;

#[allow(non_snake_case)]
mod K;

fn main() -> anyhow::Result<()> {
  // let prepare =
  //   TRT.block_on(async move { xxpg::PG.force().await.prepare(" INSERT INTO fav.user (user_id,cid,rid,ctime,action) VALUES ($1) ON CONFLICT (user_id, cid, rid, ctime) DO NOTHING RETURNING id").await.unwrap() });

  awp::init();

  let mut router = Router::new();
  macro_rules! post {
            (=> $func:ident) => {
                post!("", $func)
            };
            ($($url:ident);+) => {
                post!($($url=>$url);+)
            };
            ($($url:stmt => $func:ident);+) => {
                $(
                    post!(
                        const_str::replace!(
                            const_str::replace!(
                                const_str::unwrap!(const_str::strip_suffix!(stringify!($url), ";")),
                                " ",
                                ""
                            ),
                            "&",
                            ":"
                        ),
                        $func
                    );
                )+
            };
            ($url:expr, $func:ident) => {
                router = router.route(
                    const_str::concat!('/', $url),
                    post(FnAny($crate::url::$func::post)),
                )
            };
        }

  // get!( => stat);
  post!(li;fav;favYm);
  // post!(li => li;fav=>fav);

  router = router.route("/es/:li", get(crate::url::es::get));

  let default_port = 8879;
  let port = match std::env::var("RSRV_PORT") {
    Ok(val) => val.parse::<u16>().unwrap_or(default_port),
    _ => default_port,
  };

  TRT.block_on(async move {
    awp::srv(router.layer(middleware::from_fn(client)), port).await;
  });
  Ok(())
}
