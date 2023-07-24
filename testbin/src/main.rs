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
