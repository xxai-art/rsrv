use std::{cell::RefCell, sync::Arc};

use parking_lot::RwLock;
use tokio::time;
use tokio_postgres::{
  connect, error::SqlState, types::ToSql, Client, Error, NoTls, Row, Statement, ToStatement,
};

#[derive(Clone)]
pub struct Prepare {
  sql: Arc<Option<Statement>>,
  pg: Pg,
}

pub struct _Pg {
  pub env: String,
  pub prepare_li: Vec<Prepare>,
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
    loop {
      {
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
      }
      let env = pg.read().env.clone();
      let uri = std::env::var(&env).unwrap();

      let mut _pg = pg.write();
      if _pg._client.is_some() {
        continue;
      }

      loop {
        let mut n = 0u64;
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
                  // *arc.borrow_mut() = None;
                  pg.write()._client = None;
                  // let prepare_li: &Vec<Callback> = pg.prepare_li.borrow().as_ref();
                  // for i in prepare_li {
                  //   i(&pg);
                  // }
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
      prepare_li: Vec::new().into(),
    })))
  }

  pub async fn query_one<T>(
    &self,
    statement: &T,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<Row, Error>
  where
    T: ?Sized + ToStatement + Sync + Send,
  {
    macro_rules! query_one {
      ($client:ident) => {
        $client.query_one(statement, params)
      };
    }
    client!(self, query_one)
  }

  pub async fn query<T>(
    &self,
    statement: &T,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<Vec<Row>, Error>
  where
    T: ?Sized + ToStatement,
  {
    macro_rules! query {
      ($client:ident) => {
        $client.query(statement, params)
      };
    }
    client!(self, query)
  }

  pub async fn query_opt<T>(
    &self,
    statement: &T,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<Option<Row>, Error>
  where
    T: ?Sized + ToStatement + Sync + Send,
  {
    macro_rules! query_opt {
      ($client:ident) => {
        $client.query_opt(statement, params)
      };
    }
    client!(self, query_opt)
  }

  pub async fn execute<T>(
    &self,
    statement: &T,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<u64, Error>
  where
    T: ?Sized + ToStatement,
  {
    macro_rules! execute {
      ($client:ident) => {
        $client.execute(statement, params)
      };
    }
    client!(self, execute)
  }

  pub async fn prepare(&self, query: &str) -> Result<Prepare, Error> {
    macro_rules! prepare {
      ($client:ident) => {
        async {
          // let statement = Some($client.prepare(query).await?).into();
          let prepare = Prepare {
            sql: Arc::new(None),
            pg: self.clone(),
          };
          let pg = self.clone();
          let prepare_clone = prepare.clone();
          tokio::spawn(async move {
            pg.0.write().prepare_li.push(prepare_clone);
          });
          Ok(prepare)
        }
      };
    }
    client!(self, prepare)
  }
}
