use anyhow::Result;
use gt::GQ;

/*
CREATE TABLE IF NOT EXISTS seen (
uid BIGINT NULL,
cid TINYINT NULL,
rid BIGINT NULL,
ts TIMESTAMP(3) NOT NULL,
TIME INDEX (ts),
PRIMARY KEY (uid, cid, rid)
)
*/

pub fn after_ts_sql(uid: u64, ts: u64) -> String {
  format!("SELECT cid,rid,CAST(ts as BIGINT) t FROM seen WHERE uid={uid} AND ts>{ts} ORDER BY TS")
}

pub async fn after_ts(sql: impl AsRef<str>) -> Result<Vec<u64>> {
  let mut r = Vec::new();
  for i in GQ(sql.as_ref(), &[]).await? {
    let cid: i8 = i.get(0);
    r.push(cid as u64);
    let rid: i64 = i.get(1);
    r.push(rid as u64);
    let ts: i64 = i.get(2);
    r.push(ts as u64);
  }
  Ok(r)
}
