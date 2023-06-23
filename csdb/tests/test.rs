use csdb::{conn_by_env, Db, SQL};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DB: Db = conn_by_env("CERESDB_GRPC").unwrap();
    pub static ref SQL_DROP_TEST: SQL = DB.sql(["test"], "DROP TABLE test");

    // ctime 是用户记录创建时间
    // ts 是写入时间
    pub static ref SQL_TEST: SQL = DB.sql(["test"], r#"CREATE TABLE test (
  ts TIMESTAMP NOT NULL,
  uid uint64 NOT NULL,
  tag string NOT NULL,
  TIMESTAMP KEY(ts),
  PRIMARY KEY(uid, ts)
) ENGINE=Analytic WITH (
  compression='ZSTD',
  enable_ttl='false'
)"#);
    pub static ref SQL_INSERT: SQL = DB.sql(["test"], "INSERT INTO test (ts,uid,tag) VALUES ({},{},{})");
    pub static ref SQL_SELECT: SQL = DB.sql(["test"], "SELECT * FROM test");
    // pub static ref SQL_DELETE: SQL = DB.sql(["test"], "DELETE FROM test WHERE ts={} AND uid={}");
}

#[tokio::main]
#[test]
async fn main() -> anyhow::Result<()> {
  loginit::init();

  let _ = SQL_DROP_TEST.exe(()).await;
  SQL_TEST.exe(()).await?;
  SQL_INSERT.exe((1, 2, "test")).await?;
  SQL_INSERT.exe((2, 2, "\'\"\r\n")).await?;

  let li = SQL_SELECT.li(()).await?;
  assert_eq!(li.len(), 2);
  for i in li {
    dbg!(i);
  }
  // SQL_DELETE.exe([1, 3]).await?;
  Ok(())
}
