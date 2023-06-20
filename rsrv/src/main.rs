#![feature(lazy_cell)]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![feature(let_chains)]

use axum::{middleware, routing::post, Router};
use client::client;
use trt::TRT;

mod url;

fn main() -> anyhow::Result<()> {
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
                    post($crate::url::$func::post),
                )
            };
        }

  // get!( => stat);
  post!(li;fav);
  // post!(li => li;fav=>fav);

  // router = router.route("/sampler", get(crate::url::sampler::get));

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
