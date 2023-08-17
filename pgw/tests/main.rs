use pgw::Pg;
use tokio::time;

#[tokio::test]
async fn main() -> anyhow::Result<()> {
  loginit::init();
  let pg = Pg::new("PG_URI");
  let sql = pg
    .sql("SELECT nspname FROM pg_catalog.pg_namespace LIMIT 1")
    .await?;
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
