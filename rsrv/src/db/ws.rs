use anyhow::Result;

use crate::C::WR;

mod 同步 {
  use anyhow::Result;
  use msgpacker::prelude::*;
  use x0::{fred::interfaces::SortedSetsInterface, KV};
  use xg::Q;

  const LIMIT: usize = 8192;

  Q! {
      fav_li:SELECT id,cid,rid,ts,aid FROM fav.user WHERE uid=$1 AND id>$2 ORDER BY id LIMIT 8192;
  }

  async fn seen_li(uid: u64, ts: u64) -> Result<Vec<(u64, i8, i64)>> {
    // let sql = &format!("SELECT CAST(ts as BIGINT) t,cid,rid FROM seen WHERE uid={uid} AND ts>{ts} ORDER BY ts LIMIT {LIMIT}");
    // TODO fix https://github.com/GreptimeTeam/greptimedb/issues/2026
    let sql = format!("SELECT CAST(ts as BIGINT) t,cid,rid FROM seen WHERE uid={uid} AND ts>ARROW_CAST({ts},'Timestamp(Millisecond,None)') ORDER BY ts LIMIT 8192");
    Ok(
      gt::Q(sql, &[])
        .await?
        .into_iter()
        .map(|i| (i.get::<_, i64>(0) as u64, i.get(1), i.get(2)))
        .collect(),
    )
  }

  #[derive(MsgPacker)]
  pub struct IdLi {
    pub id_li: Vec<u64>,
  }

  pub async fn run(uid: u64, channel_id: String, body: &[u8]) -> Result<()> {
    dbg!(&body);
    // let id_li = IdLi::unpack(&body)?.1.id_li;
    // dbg!(id_li);
    Ok(())
  }
}

pub async fn ws(action: WR, uid: u64, channel_id: String, body: &[u8]) -> Result<()> {
  match action {
    WR::同步 => 同步::run(uid, channel_id, body).await?,
  }
  Ok(())
}
