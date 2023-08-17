use lazy_static::lazy_static;
use pgw::{Pg, Sql};
use tokio::time;
use tokio_postgres::types::Oid;

lazy_static! {
  // get postgres connection uri from environment ( without prefix )
  static ref PG: Pg = Pg::new_with_env("PG_URI");
  // prepared sql
  static ref SQL_NSPNAME: Sql = PG.sql("SELECT oid FROM pg_catalog.pg_namespace LIMIT 2");
}

#[tokio::test]
async fn main() -> anyhow::Result<()> {
  loginit::init();
  for i in 0..99999 {
    println!("loop {i}");
    match PG.query(&*SQL_NSPNAME, &[]).await {
      Ok(li) => {
        for i in li {
          let oid: Oid = i.try_get(0)?;
          dbg!(oid);
        }
      }
      Err(err) => {
        dbg!(err);
      }
    }
    match PG
      .query_one("SELECT oid FROM pg_catalog.pg_namespace LIMIT 1", &[])
      .await
    {
      Ok(i) => {
        let oid: Oid = i.try_get(0)?;
        dbg!(oid);
      }
      Err(err) => {
        dbg!(err);
      }
    }
    time::sleep(std::time::Duration::from_secs(6)).await;
  }
  Ok(())
}
