use std::{env::var, time::Duration};

use ceresdb_client::{Builder, Mode, RpcConfig, RpcContext};
use csdb::Db;

use crate::sql::Db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  loginit::init();
  let rpc = var("CERESDB_GRPC").unwrap();
  let rpc_config = RpcConfig {
    thread_num: Some(1),
    default_write_timeout: Duration::from_millis(1000),
    ..Default::default()
  };

  let builder = Builder::new(rpc, Mode::Direct)
    .rpc_config(rpc_config)
    .default_database("public");

  let client = builder.build();
  let ctx = RpcContext::default();
  let db = Db::new(ctx, client);

  db.sql(["fav"], "DROP TABLE fav").noerr_nort().await;

  // id 是用户记录创建时间
  // ts 是写入时间
  let fav = r#"CREATE TABLE fav (
  ts TIMESTAMP NOT NULL,
  ctime uint64 NOT NULL,
  uid uint64 NOT NULL,
  action uint8 NOT NULL,
  kind uint8 NOT NULL,
  rid uint64 NOT NULL,
  TIMESTAMP KEY(ts),
  PRIMARY KEY(uid, ts)
) ENGINE=Analytic WITH (
  compression='ZSTD',
  enable_ttl='false'
);"#;

  db.sql(["fav"], fav).noerr_nort().await;

  Ok(())
}
