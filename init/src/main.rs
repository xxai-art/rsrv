mod sql;
use std::{env::var, time::Duration};

use ceresdb_client::{Builder, Mode, RpcConfig, RpcContext};

use crate::sql::Db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let rpc = var("CERESDB_GRPC").unwrap();
  let rpc_config = RpcConfig {
    thread_num: Some(1),
    default_write_timeout: Duration::from_millis(1000),
    ..Default::default()
  };

  let builder = Builder::new(rpc, Mode::Direct)
    .rpc_config(rpc_config)
    .default_database("art");

  let client = builder.build();
  let ctx = RpcContext::default();
  let db = Db::new(ctx, client);

  let fav = r#"CREATE TABLE fav (
  ts TIMESTAMP NOT NULL,
  id uint64 NOT NULL,
  uid uint64 NOT NULL,
  action uint8 NOT NULL,
  kind uint8 NOT NULL,
  rid uint64 NOT NULL,
  TIMESTAMP KEY(ts),
  PRIMARY KEY(id)
) ENGINE=Analytic WITH (
  compression='ZSTD',
  enable_ttl=false
);"#;

  let sql = db.sql(["fav"], fav);
  let q = sql.exe().await?;
  dbg!(q);
  Ok(())
}
