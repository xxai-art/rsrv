use csdb::{conn_by_env, Db, Sql};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DB: Db = conn_by_env("CERESDB_GRPC").unwrap();
    pub static ref SQL_DROP_FAV: Sql<'static> = DB.sql(["fav"], "DROP TABLE fav");

    // ctime 是用户记录创建时间
    // ts 是写入时间
    pub static ref SQL_FAV: Sql<'static> = DB.sql(["fav"], r#"CREATE TABLE fav (
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
)"#);

}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  loginit::init();

  SQL_DROP_FAV.ignore_err_no_return().await;
  SQL_FAV.ignore_err_no_return().await;

  Ok(())
}
