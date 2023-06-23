use std::env::var;

use ceresdb_client::{Builder, Mode, RpcConfig, RpcContext};
use csdb::{conn_by_env, Db};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  loginit::init();

  let db = conn_by_env("CERESDB_GRPC")?;

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
