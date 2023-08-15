use anyhow::Error;
use gt::GQ;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let sql_li = [
    r#"CREATE TABLE IF NOT EXISTS seen (
uid BIGINT UNSIGNED NOT NULL,
cid TINYINT UNSIGNED NOT NULL,
rid BIGINT UNSIGNED NOT NULL,
ts TIMESTAMP(3) NOT NULL,
TIME INDEX (ts),
PRIMARY KEY (uid, cid, rid)
)"#,
    r#"CREATE TABLE IF NOT EXISTS log (
uid BIGINT UNSIGNED NOT NULL,
aid TINYINT UNSIGNED NOT NULL,
cid TINYINT UNSIGNED NOT NULL,
rid BIGINT UNSIGNED NOT NULL,
q BIGINT UNSIGNED NOT NULL,
ts TIMESTAMP(3) NOT NULL,
TIME INDEX (ts),
PRIMARY KEY (uid, aid, cid, rid, q)
)"#,
    r#"CREATE TABLE IF NOT EXISTS q (
id TIMESTAMP(3) NOT NULL,
q STRING NOT NULL,
TIME INDEX (id),
PRIMARY KEY (q)
)"#,
  ];
  for sql in sql_li {
    println!("{}", sql);
    GQ(sql, &[]).await?;
  }
  Ok(())
}
