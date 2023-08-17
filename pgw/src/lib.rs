#![feature(async_fn_in_trait)]

use std::sync::Arc;

use parking_lot::RwLock;
use tokio::time;
use tokio_postgres::{
  connect, error::SqlState, types::ToSql, Client, Error, NoTls, Row, Statement,
};

pub struct Sql {
  sql: String,
  st: RwLock<Option<Statement>>,
  pg: Pg,
}

pub trait IntoStatement {
  async fn into(self) -> Result<Statement, Error>;
}

impl IntoStatement for &Arc<Sql> {
  async fn into(self) -> Result<Statement, Error> {
    loop {
      if let Some(st) = self.st.read().as_ref() {
        return Ok(st.clone());
      }

      dbg!("wait prepare");
      let st = self.pg.prepare(&*self.sql).await?;
      *self.st.write() = Some(st);
    }
  }
}

pub struct _Pg {
  pub env: String,
  pub sql_li: Vec<Arc<Sql>>,
  _client: Option<Client>,
}

#[derive(Clone)]
pub struct Pg(Arc<RwLock<_Pg>>);

fn is_close(err: &Error, err_code: Option<&SqlState>) -> bool {
  err_code == Some(&SqlState::ADMIN_SHUTDOWN) || err.is_closed()
}

macro_rules! client {
  ($self:ident, $body:ident) => {{
    let pg = &$self.0;
    'outer: loop {
      {
        dbg!("pg read 1");
        if let Some(client) = &pg.read()._client {
          loop {
            match $body!(client).await {
              Ok(r) => return Ok(r),
              Err(err) => {
                if is_close(&err, err.code()) {
                  break;
                }
                return Err(err);
              }
            }
          }
        }
        dbg!("pg read end 1");
      }
      let env = { pg.read().env.clone() };
      let uri = std::env::var(&env).unwrap();

      loop {
        let mut n = 0u64;
        dbg!("conn write");
        let mut _pg = pg.write();
        if _pg._client.is_some() {
          continue 'outer;
        }
        match connect(&format!("postgres://{}", uri), NoTls).await {
          Ok((client, connection)) => {
            _pg._client = Some(client);

            let pg = pg.clone();
            tokio::spawn(async move {
              if let Err(e) = connection.await {
                let err_code = e.code();
                let code = match err_code {
                  Some(code) => code.code(),
                  None => "",
                };
                tracing::error!("❌ {env} ERROR CODE {code} : {e}");

                if is_close(&e, err_code) {
                  let mut pg = pg.write();
                  pg._client = None;
                  for i in &mut pg.sql_li {
                    *i.st.write() = None;
                  }
                  return;
                }
              }
            });
            break;
          }
          Err(err) => {
            n += 1;
            tracing::error!("❌ RETRY {n} → {env} : {err}");
            time::sleep(std::time::Duration::from_secs(3)).await;
          }
        }
      }
    }
  }};
}

impl Pg {
  pub fn new(env: impl Into<String>) -> Self {
    Self(Arc::new(RwLock::new(_Pg {
      env: env.into(),
      _client: None,
      sql_li: Vec::new().into(),
    })))
  }

  pub async fn query_one(
    &self,
    statement: &Statement,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<Row, Error> {
    macro_rules! query_one {
      ($client:ident) => {
        $client.query_one(statement, params)
      };
    }
    client!(self, query_one)
  }

  pub async fn query(
    &self,
    statement: impl IntoStatement,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<Vec<Row>, Error> {
    let statement = statement.into().await?;
    macro_rules! query {
      ($client:ident) => {
        $client.query(&statement, params)
      };
    }
    client!(self, query)
  }

  pub async fn query_opt(
    &self,
    statement: impl IntoStatement,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<Option<Row>, Error> {
    let statement = statement.into().await?;
    macro_rules! query_opt {
      ($client:ident) => {
        $client.query_opt(&statement, params)
      };
    }
    client!(self, query_opt)
  }

  pub async fn execute(
    &self,
    statement: impl IntoStatement,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<u64, Error> {
    let statement = statement.into().await?;
    macro_rules! execute {
      ($client:ident) => {
        $client.execute(&statement, params)
      };
    }
    client!(self, execute)
  }

  pub async fn prepare(&self, query: impl AsRef<str>) -> Result<Statement, Error> {
    macro_rules! prepare {
      ($client:ident) => {
        $client.prepare(query.as_ref())
      };
    }
    client!(self, prepare)
  }

  pub async fn sql(&self, query: impl Into<String>) -> Result<Arc<Sql>, Error> {
    let sql = Arc::new(Sql {
      sql: query.into(),
      st: RwLock::new(None),
      pg: self.clone(),
    });
    {
      self.0.write().sql_li.push(sql.clone())
    }
    macro_rules! sql {
      ($client:ident) => {{
        async {
          let pg = self.clone();
          Ok(sql.clone())
        }
      }};
    }
    client!(self, sql)
  }
}
