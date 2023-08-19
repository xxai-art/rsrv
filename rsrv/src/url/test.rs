use anypack::{Any, VecAny};
use awp::any;
use axum::body::Bytes;
use x0::{fred::interfaces::HashesInterface, R};
use intbin::u64_bin;
use xxpg::{Pg, Q};

Q!(
li:
    SELECT cid,rid,ts FROM fav.user LIMIT 2
);

pub async fn post(body: Bytes) -> any!() {
  Ok(li().await?)
}
