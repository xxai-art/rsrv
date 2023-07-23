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
// q!(Q1, query_one, Row);
// q!(Q01, query_opt, Option<Row>);

#[tokio::main]
async fn main() -> Result<(), Error> {
  use std::future::IntoFuture;
  GT.into_future().await;
  let rows = G(
    &format!(
      "select uid,cid,rid,CAST(ts AS BIGINT) t FROM seen WHERE uid={} AND ts>{} ORDER BY ts",
      1, 2
    ),
    &[],
  )
  .await?;

  dbg!(&rows);
  let uid: i64 = rows[0].get(0);
  let cid: i8 = rows[0].get(1);
  let rid: i64 = rows[0].get(2);
  let ts: i64 = rows[0].get(3);

  dbg!(uid, cid, rid, ts);
  Ok(())
}
