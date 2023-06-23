use std::{
  env::{var, VarError},
  fmt::{Display, Formatter},
  sync::Arc,
};

use aho_corasick::AhoCorasick;
use ceresdb_client::{
  model::sql_query::row::Row, Builder, DbClient, Error, Mode, RpcConfig, RpcContext,
  SqlQueryRequest, SqlQueryResponse,
};
use coarsetime::Instant;
use dyn_fmt::AsStrFormatExt;
use lazy_static::lazy_static;
use tracing::{error, info};

pub static NONE: [u8; 0] = [];

pub type SQL = Sql<'static>;

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
#[derive(Debug, Clone)]
pub struct Val(String);

impl Display for Val {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "{}", self.0)
  }
}

macro_rules! into_val {
    ($($ty:ty),+) => {
        $(
            impl From<$ty> for Val {
                fn from(v: $ty) -> Self {
                    Val(v.to_string())
                }
            }
            impl From<&$ty> for Val {
                fn from(v: &$ty) -> Self {
                    Val(v.to_string())
                }
            }
        )+
    };
}

into_val!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

pub static REPLACE: [&str; 4] = ["\\", "\'", "\r", "\n"];
pub static REPLACE_WITH: [&str; 4] = ["\\\\", "\\\'", "\\r", "\\n"];

lazy_static! {
  static ref AC: AhoCorasick = AhoCorasick::new(REPLACE).unwrap();
}

impl From<&str> for Val {
  fn from(v: &str) -> Self {
    let mut wtr = vec![];
    AC.try_stream_replace_all(v.as_bytes(), &mut wtr, &REPLACE_WITH)
      .unwrap();

    wtr.push(b'\'');
    wtr.insert(0, b'\'');

    Val(String::from_utf8_lossy(&wtr).into())
  }
}

impl From<String> for Val {
  fn from(v: String) -> Self {
    v.as_str().into()
  }
}

pub struct ValLi(pub Vec<Val>);

impl<T: IntoIterator<Item = V>, V: Into<Val>> From<T> for ValLi {
  fn from(iter: T) -> Self {
    ValLi(iter.into_iter().map(|i| i.into()).collect())
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
    info!("{}ms\n{}", cost, &self.sql);
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
