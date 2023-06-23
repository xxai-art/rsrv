mod queryer;
use std::{env::var, pin::Pin, sync::Arc, time::Duration};

use ceresdb_client::{Builder, DbClient, Mode, RpcConfig, RpcContext, SqlQueryRequest};

use crate::queryer::Queryer;

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

  let ctx = RpcContext::default();

  let q = Queryer { ctx, client };
  let r = q.query(["fav"], fav).await;
  dbg!(r);
  Ok(())
}
