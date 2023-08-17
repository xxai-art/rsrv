use anypack::{Any, VecAny};
use awp::any;
use axum::body::Bytes;
use x0::{fred::interfaces::HashesInterface, R};
use xxai::u64_bin;
use xxpg::{Pg, Q};
//
// Q!(
// li:
//     SELECT cid,rid,ts FROM fav.user LIMIT 2
// );

pub async fn li() -> anyhow::Result<Vec<(u16, u64, u64)>> {
  let r = xxpg::PG.query_one(&"select 1", &[]).await.unwrap();
  // .unwrap()
  // .iter()
  // .map(|r| (r.get::<_, u16>(0), r.get::<_, u64>(1), r.get::<_, u64>(2)))
  // .collect();
  Ok(vec![])
}

pub async fn post(body: Bytes) -> any!() {
  // Ok(li().await?)
  // let x = li().await.unwrap();
  trt::spawn!({
    let pg = Pg::new_with_env("PG_URI");
    let r = pg.query_one(&"select 1", &[]).await.unwrap();
  });
  Ok(0)
}
