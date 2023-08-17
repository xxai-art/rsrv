use std::fmt::{Debug, Formatter};

// pub use async_lazy;
// use async_lazy::Lazy;
// pub use ctor::ctor;
// pub use paste::paste;
// use tokio_postgres::ToStatementType;
// pub use tokio_postgres::{self, types::ToSql, Client, Error, NoTls, Row, Statement, ToStatement};
// use tracing::error;
// pub use trt::TRT;
// pub use xxpg_proc::{Q, Q01, Q1};

// pub struct LazyStatement {
//   pub statement: async_lazy::Lazy<tokio_postgres::Statement>,
//   pub sql: &'static str,
// }
//
// impl Debug for LazyStatement {
//   fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
//     fmt.write_str(self.sql)?;
//     Ok(())
//   }
// }
//
// impl ToStatement for LazyStatement {
//   fn __convert(&self) -> ToStatementType<'_> {
//     ToStatementType::Statement(self.statement.get().unwrap())
//   }
// }
//
// #[macro_export]
// macro_rules! sql {
//     ($($var:ident : $sql:expr),+ ) => {
//         $(
//             $crate::paste!{
//                 pub static [<__ $var:upper >]: $crate::LazyStatement  =
//                     $crate::LazyStatement{
//                         statement:$crate::async_lazy::Lazy::const_new(|| Box::pin(async move { $crate::PG.force().await.prepare($sql).await.unwrap() })),
//                         sql:$sql
//                     };
//                 pub static [<$var:upper>] : &$crate::LazyStatement  = &[<__ $var:upper>];
//             }
//         )+
//
//        $crate::paste!{
//            mod [<private $(_$var )+>] {
//                #[$crate::ctor]
//                fn pg_statement_init() {
//                    $crate::TRT.block_on(async move {
//                        $(super::[<$var:upper>].statement.force().await;)+
//                    });
//                }
//            }
//        }
//     };
// }
//
// //   r = ONE0"SELECT name FROM img.sampler WHERE id=#{id}"
// // else
// //   r = await LI"SELECT id,name FROM img.sampler"
//
// pub async fn conn() -> Client {
//   pgw::conn("PG_URI").await
// }
//
// pub static PG: Lazy<Client> = Lazy::const_new(|| Box::pin(async move { conn().await }));
//
// #[ctor]
// fn init() {
//   TRT.block_on(async move {
//     use std::future::IntoFuture;
//     PG.into_future().await;
//   });
// }
//
macro_rules! q {
  ($name:ident,$func:ident,$rt:ty) => {
    #[allow(non_snake_case)]
    pub async fn $name<T>(statement: &T, params: &[&(dyn ToSql + Sync)]) -> Result<$rt, Error>
    where
      T: ?Sized + ToStatement + Debug,
    {
      match PG.get().unwrap().$func(statement, params).await {
        Ok(r) => Ok(r),
        Err(err) => {
          if err.is_closed() {
            error!("{:?}\n{}", statement, err);
            std::process::exit(1);
          }
          Err(err)
        }
      }
    }
  };
}
//
// q!(Q, query, Vec<Row>);
// q!(Q1, query_one, Row);
// q!(Q01, query_opt, Option<Row>);
