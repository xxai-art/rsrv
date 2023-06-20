pub use async_lazy;
use async_lazy::Lazy;
pub use ctor::ctor;
pub use paste::paste;
use tokio_postgres::ToStatementType;
pub use tokio_postgres::{self, types::ToSql, Client, Error, NoTls, Row, Statement, ToStatement};
pub use trt::TRT;
pub use xxpg_proc::{Q, Q1};

pub struct LazyStatement(pub async_lazy::Lazy<tokio_postgres::Statement>);

impl ToStatement for LazyStatement {
  fn __convert(&self) -> ToStatementType<'_> {
    ToStatementType::Statement(self.0.get().unwrap())
  }
}

#[macro_export]
macro_rules! sql {
    ($($var:ident : $sql:expr),+ ) => {
        $(
            $crate::paste!{
                pub static [<__ $var:upper >]: $crate::LazyStatement  =
                    $crate::LazyStatement($crate::async_lazy::Lazy::const_new(|| Box::pin(async move { $crate::PG.force().await.prepare($sql).await.unwrap() })));
                pub static [<$var:upper>] : &$crate::LazyStatement  = &[<__ $var:upper>];
            }
        )+

            mod private {
                #[$crate::ctor]
                fn pg_statement_init() {
                    $crate::TRT.block_on(async move {
                        $crate::paste!{
                            $(super::[<$var:upper>].0.force().await;)+
                        }
                    });
                }
            }
    };
}

//   r = ONE0"SELECT name FROM img.sampler WHERE id=#{id}"
// else
//   r = await LI"SELECT id,name FROM img.sampler"

pub static PG: Lazy<Client> = Lazy::const_new(|| {
  let pg_uri = std::env::var("PG_URI").unwrap();
  Box::pin(async move {
    let (client, connection) = tokio_postgres::connect(&format!("postgres://{}", pg_uri), NoTls)
      .await
      .unwrap();
    tokio::spawn(async move {
      if let Err(e) = connection.await {
        tracing::error!("postgres connection error: {e}");
      }
    });

    client
  })
});

#[ctor]
fn init() {
  TRT.block_on(async move {
    use std::future::IntoFuture;
    PG.into_future().await;
  });
}

#[allow(non_snake_case)]
pub async fn Q<T>(statement: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error>
where
  T: ?Sized + ToStatement,
{
  PG.get().unwrap().query(statement, params).await
}

#[allow(non_snake_case)]
pub async fn Q1<T>(statement: &T, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error>
where
  T: ?Sized + ToStatement,
{
  PG.get().unwrap().query_one(statement, params).await
}
