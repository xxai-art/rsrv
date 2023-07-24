use async_lazy::Lazy;
use tokio_postgres::{types::ToSql, Client, Error, NoTls, Row, ToStatement};

static GT: Lazy<Client> = Lazy::const_new(|| Box::pin(async move { pgw::conn("GT_URI").await }));

macro_rules! q {
  ($name:ident,$func:ident,$rt:ty) => {
    #[allow(non_snake_case)]
    pub async fn $name<T>(statement: &T, params: &[&(dyn ToSql + Sync)]) -> Result<$rt, Error>
    where
      T: ?Sized + ToStatement,
    {
      match GT.get().unwrap().$func(statement, params).await {
        Ok(r) => Ok(r),
        Err(err) => {
          if err.is_closed() {
            tracing::error!("{}", err);
            std::process::exit(1);
          }
          Err(err)
        }
      }
    }
  };
}

q!(G, query, Vec<Row>);
