lazy_static! {
  pub static ref CSDB: Db = conn_by_env("CERESDB_GRPC").unwrap();
}
