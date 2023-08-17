Base on tokio postgres

Keep one postgres connection, will auto reconnect when connect close

基于 tokio postgres , 保留一个 postgres 连接 , 连接关闭时会自动重新连接

[→ tests/main.rs](tests/main.rs)

```rust
use lazy_static::lazy_static;
use pgw::{Pg, Sql};
use tokio::time;
use tokio_postgres::types::Oid;

lazy_static! {
  static ref PG: Pg = Pg::new("PG_URI");
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
```

