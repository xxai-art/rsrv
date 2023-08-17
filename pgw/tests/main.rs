use lazy_static::lazy_static;
use pgw::Pg;
use tokio::time;

lazy_static! {
  static ref PG: Pg = Pg::new("PG_URI");
}

#[tokio::test]
async fn main() -> anyhow::Result<()> {
  loginit::init();
  let pg = Pg::new("PG_URI");
  let sql = pg.sql("SELECT nspname FROM pg_catalog.pg_namespace LIMIT 1");
  for i in 0..99999 {
    match pg.query(&sql, &[]).await {
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
