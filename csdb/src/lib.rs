use std::{
  env::{var, VarError},
  sync::Arc,
};

use ceresdb_client::{
  Builder, DbClient, Error, Mode, RpcConfig, RpcContext, SqlQueryRequest, SqlQueryResponse,
};
use coarsetime::Instant;

pub fn conn_by_env(env: impl AsRef<str>) -> Result<Db, VarError> {
  let grpc = var(env.as_ref())?;
  Ok(conn(grpc))
}

pub fn conn(grpc: impl Into<String>) -> Db {
  let rpc_config = RpcConfig::default();

  let builder = Builder::new(grpc.into(), Mode::Direct)
    .rpc_config(rpc_config)
    .default_database("public");

  let client = builder.build();
  let ctx = RpcContext::default();
  Db::new(ctx, client)
}

pub struct Sql<'a> {
  pub req: SqlQueryRequest,
  pub db: &'a Db,
}

pub struct Db {
  pub ctx: RpcContext,
  pub client: Arc<dyn DbClient>,
}

impl Db {
  pub fn new(ctx: RpcContext, client: Arc<dyn DbClient>) -> Self {
    Db { ctx, client }
  }

  pub fn sql(&self, tables: impl Into<Tables>, sql: impl Into<String>) -> Sql {
    let req = SqlQueryRequest {
      tables: tables.into().0,
      sql: sql.into(),
    };
    Sql { db: self, req }
  }
}

pub struct Tables(pub Vec<String>);

impl From<Vec<String>> for Tables {
  fn from(v: Vec<std::string::String>) -> Self {
    Tables(v)
  }
}

impl<const N: usize> From<[&str; N]> for Tables {
  fn from(v: [&str; N]) -> Self {
    Tables(v.map(|i| i.to_string()).into_iter().collect())
  }
}

impl<'a> Sql<'a> {
  pub async fn noerr_nort(&self) {
    self.noerr().await;
  }

  pub async fn noerr(&self) -> Option<SqlQueryResponse> {
    match self.exe().await {
      Ok(r) => return Some(r),
      Err(err) => match err {
        Error::Server(e) => {
          eprintln!("CERESDB ERROR CODE {}:\n{}\n", e.code, e.msg);
        }
        _ => {
          eprintln!("{err}");
        }
      },
    };
    None
  }

  pub async fn exe(&self) -> Result<SqlQueryResponse, Error> {
    let db = &self.db;
    let timer = Instant::now();
    let r = db.client.sql_query(&db.ctx, &self.req).await;
    let cost = timer.elapsed().as_millis();
    tracing::info!("{}ms\n{}", cost, self.req.sql);
    r
  }
}
