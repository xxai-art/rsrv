use anyhow::Error;
use gt::GQ;

#[tokio::main]
async fn main() -> Result<(), Error> {
  let seen = r#"CREATE TABLE IF NOT EXISTS seen (
uid BIGINT NULL,
cid TINYINT NULL,
rid BIGINT NULL,
ts TIMESTAMP(3) NOT NULL,
TIME INDEX (ts),
PRIMARY KEY (uid, cid, rid)
)"#;
  GQ(seen, &[]).await?;
  Ok(())
}
