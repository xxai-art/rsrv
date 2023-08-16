use pgw::Pg;
use tokio::time;

#[tokio::test]
async fn main() -> anyhow::Result<()> {
  loginit::init();
  let pg = Pg::new("PG_URI");
  for i in 0..9999999 {
    match pg
      .query("SELECT nspname FROM pg_catalog.pg_namespace LIMIT 1", &[])
      .await
    {
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
