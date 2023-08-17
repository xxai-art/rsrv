use std::sync::Arc;

use parking_lot::RwLock;
use tokio::time;
use tokio_postgres::{
  connect, error::SqlState, types::ToSql, Client, Error, NoTls, Row, ToStatement,
};

pub struct Pg {
  pub env: String,
  _client: Arc<RwLock<Option<Client>>>,
}

fn is_close(err: &Error, err_code: Option<&SqlState>) -> bool {
  err_code == Some(&SqlState::ADMIN_SHUTDOWN) || err.is_closed()
}

macro_rules! client {
  ($self:ident, $body:ident) => {{
    loop {
      if let Some(client) = &*$self._client.read() {
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
      let env = $self.env.clone();
      let uri = std::env::var(&env).unwrap();
      let mut _client = $self._client.write();
      if _client.is_some() {
        continue;
      }
      let mut n = 0u64;
      loop {
        match connect(&format!("postgres://{}", uri), NoTls).await {
          Ok((client, connection)) => {
            *_client = Some(client);

            let arc = $self._client.clone();
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
                  *arc.write() = None;
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
  // let pg_uri = std::env::var(&env).unwrap();
  pub fn new(env: impl Into<String>) -> Self {
    Self {
      env: env.into(),
      _client: RwLock::new(None).into(),
    }
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
}

// pub async fn conn(env: impl Into<String>) -> Pg {
//   let env = env.into();
//   let pg_uri = std::env::var(&env).unwrap();
//   let (client, connection) = tokio_postgres::connect(&format!("postgres://{}", pg_uri), NoTls)
//     .await
//     .unwrap();
//
//   let mut pg = Pg {
//     client: Some(client),
//   };
//
//   tokio::spawn(async move {
//     if let Err(e) = connection.await {
//       let err_code = e.code();
//       let code = match err_code {
//         Some(code) => code.code(),
//         None => "",
//       };
//       tracing::error!("❌ {env} ERROR CODE {code} : {e}");
//
//       if err_code == Some(&SqlState::ADMIN_SHUTDOWN) || e.is_closed() {
//         std::process::exit(1)
//       }
//     }
//   });
//   pg
// }
