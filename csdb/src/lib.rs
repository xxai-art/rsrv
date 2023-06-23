use std::sync::Arc;

use ceresdb_client::{DbClient, Error, RpcContext, SqlQueryRequest, SqlQueryResponse};

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
    tracing::info!("{}", self.req.sql);
    db.client.sql_query(&db.ctx, &self.req).await
  }
}
