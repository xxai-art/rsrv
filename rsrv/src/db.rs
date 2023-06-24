lazy_static! {
  pub static ref CSDB: Db = conn_by_env("CERESDB_GRPC").unwrap();
  pub static ref SQL_INSERT: SQL =
    DB.sql("INSERT INTO fav (ts,ctime,uid,action,kind,rid) VALUES ({},{},{},{},{},{})");
}
