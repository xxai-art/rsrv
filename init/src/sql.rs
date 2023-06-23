use std::sync::Arc;

use ceresdb_client::{DbClient, Error, RpcContext, SqlQueryRequest, SqlQueryResponse};

pub struct Sql {
  pub ctx: RpcContext,
  pub client: Arc<dyn DbClient>,
  pub req: SqlQueryRequest,
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

impl Sql {
  pub fn new(
    ctx: RpcContext,
    client: Arc<dyn DbClient>,
    tables: impl Into<Tables>,
    sql: impl Into<String>,
  ) -> Self {
    let req = SqlQueryRequest {
      tables: tables.into().0,
      sql: sql.into(),
    };
    Sql { ctx, client, req }
  }

  pub async fn exe(&self) -> Result<SqlQueryResponse, Error> {
    self.client.sql_query(&self.ctx, &self.req).await
  }
}

// impl Sql {
//   pub async fn query(
//     &self,
//     tables: impl Into<Tables>,
//     sql: impl Into<String>,
//   ) -> Result<SqlQueryResponse, Error> {
//     let req = SqlQueryRequest {
//       tables: tables.into().0,
//       sql: sql.into(),
//     };
//     self.client.sql_query(&self.ctx, &req).await
//   }
// }
