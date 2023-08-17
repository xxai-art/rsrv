use lazy_static::lazy_static;
use pgw::{Pg, Sql};
use tokio::time;

lazy_static! {
  static ref PG: Pg = Pg::new("PG_URI");
  static ref SQL_NSPNAME: Sql = PG.sql("SELECT nspname FROM pg_catalog.pg_namespace LIMIT 1");
}

#[tokio::test]
async fn main() -> anyhow::Result<()> {
  loginit::init();
  for i in 0..99999 {
    match PG.query(&*SQL_NSPNAME, &[]).await {
      Ok(li) => {
        dbg!(i, li);
      }
      Err(err) => {
        dbg!(err);
      }
    }
    time::sleep(std::time::Duration::from_secs(6)).await;
  }
  Ok(())
}
