mod val;
mod val_li;

use std::{
  env::{var, VarError},
  fmt::{Display, Formatter},
  sync::Arc,
};

use aho_corasick::AhoCorasick;
mod into_val_li;
use ceresdb_client::{
  model::sql_query::row::Row, Builder, DbClient, Error, Mode, RpcConfig, RpcContext,
  SqlQueryRequest, SqlQueryResponse,
};
use coarsetime::Instant;
use dyn_fmt::AsStrFormatExt;
use lazy_static::lazy_static;
use tracing::{error, info};

pub use crate::{val::Val, val_li::ValLi};

pub type SQL = Sql<'static>;

pub fn conn_by_env(env: impl AsRef<str>) -> Result<Db, VarError> {
  let grpc = var(env.as_ref())?;
  Ok(conn(grpc))
}

pub fn conn(grpc: impl Into<String>) -> Db {
  let rpc_config = RpcConfig::default();

  let builder = Builder::new(grpc.into(), Mode::Direct)
    .rpc_config(rpc_config)
    .default_database("public".to_string());

  let client = builder.build();
  let ctx = RpcContext::default();
  Db::new(ctx, client)
}

pub struct Sql<'a> {
  pub sql: String,
  pub db: &'a Db,
  pub tables: Vec<String>,
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
    Sql {
      db: self,
      sql: sql.into(),
      tables: tables.into().0,
    }
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
  pub async fn li(&self, args: impl Into<ValLi>) -> Result<Vec<Row>, Error> {
    Ok(self.exe(args).await?.rows)
  }

  pub async fn exe(&self, args: impl Into<ValLi>) -> Result<SqlQueryResponse, Error> {
    let timer = Instant::now();
    let db = &self.db;

    let args = args.into().0;

    let sql = if args.len() > 0 {
      self.sql.format(&args)
    } else {
      self.sql.to_string()
    };

    let req = SqlQueryRequest {
      tables: self.tables.clone(),
      sql,
    };
    let r = db.client.sql_query(&db.ctx, &req).await;
    let cost = timer.elapsed().as_millis();
    info!("{}ms {}", cost, &req.sql);
    if let Err(err) = &r {
      match err {
        Error::Server(e) => {
          error!("CERESDB ERROR CODE {}:\n{}\n", e.code, e.msg);
        }
        _ => {
          error!("{err}");
        }
      }
    }
    r
  }
}
