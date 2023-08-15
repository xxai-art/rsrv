use anyhow::Error;
use gt::GQ;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let sql_li = [
    r#"CREATE TABLE IF NOT EXISTS seen (
uid BIGINT NOT NULL,
cid TINYINT NOT NULL,
rid BIGINT NOT NULL,
ts TIMESTAMP(3) NOT NULL,
TIME INDEX (ts),
PRIMARY KEY (uid, cid, rid)
)"#,
    r#"CREATE TABLE IF NOT EXISTS log (
uid BIGINT NOT NULL,
aid TINYINT NOT NULL,
cid TINYINT NOT NULL,
rid BIGINT NOT NULL,
ts TIMESTAMP(3) NOT NULL,
TIME INDEX (ts),
PRIMARY KEY (aid, cid, rid,  uid)
)"#,
  ];
  for sql in sql_li {
    println!("{}", sql);
    GQ(sql, &[]).await?;
  }
  Ok(())
}
