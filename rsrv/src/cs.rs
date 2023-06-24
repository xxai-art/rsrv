use csdb::{conn_by_env, Db, SQL};
use lazy_static::lazy_static;

lazy_static! {
  pub static ref CS: Db = conn_by_env("CERESDB_GRPC").unwrap();
  pub static ref SQL_INSERT: SQL =
    CS.sql("INSERT INTO fav (ts,ctime,uid,action,kind,rid) VALUES ({},{},{},{},{},{})");
}
