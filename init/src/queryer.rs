use std::{env::var, future::Future, pin::Pin, sync::Arc, time::Duration};

use ceresdb_client::{
  Builder, DbClient, Error, Mode, RpcConfig, RpcContext, SqlQueryRequest, SqlQueryResponse,
};

pub struct Queryer {
  pub ctx: RpcContext,
  pub client: Arc<dyn DbClient>,
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

impl Queryer {
  pub async fn query(
    &self,
    tables: impl Into<Tables>,
    sql: impl Into<String>,
  ) -> Result<SqlQueryResponse, Error> {
    let req = SqlQueryRequest {
      tables: tables.into().0,
      sql: sql.into(),
    };
    Ok(self.client.sql_query(&self.ctx, &req).await?)
  }
}
