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
