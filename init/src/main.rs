use csdb::{conn_by_env, Db, NONE, SQL};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DB: Db = conn_by_env("CERESDB_GRPC").unwrap();
    pub static ref SQL_DROP_FAV: SQL = DB.sql(["fav"], "DROP TABLE fav");

    // ctime 是用户记录创建时间
    // ts 是写入时间
    pub static ref SQL_FAV: SQL = DB.sql(["fav"], r#"CREATE TABLE fav (
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
    pub static ref SQL_INSERT: SQL = DB.sql(["fav"], "INSERT INTO fav (ts,ctime,uid,action,kind,rid) VALUES ({},{},{},{},{},{})");
    pub static ref SQL_SELECT: SQL = DB.sql(["fav"], "SELECT * FROM fav");
    // pub static ref SQL_DELETE: SQL = DB.sql(["fav"], "DELETE FROM fav WHERE ts={} AND uid={}");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  loginit::init();

  let _ = SQL_DROP_FAV.exe(NONE).await;
  SQL_FAV.exe(NONE).await?;
  SQL_INSERT.exe([1, 2, 3, 4, 5, 6]).await?;
  SQL_INSERT.exe([2, 2, 3, 4, 5, 6]).await?;

  let li = SQL_SELECT.li(NONE).await?;
  for i in li {
    dbg!(&i);
  }
  // SQL_DELETE.exe([1, 3]).await?;
  Ok(())
}
