use anyhow::Result;

use crate::C::WR;

mod 同步 {
  use anyhow::Result;
  use anypack::{Any, Pack, VecAny};
  use msgpacker::prelude::*;
  use x0::{fred::interfaces::SortedSetsInterface, KV};
  use xg::Q;

  use crate::C::WS;

  const LIMIT: usize = 8192;

  Q! {
      fav_li:SELECT id,cid,rid,ts,aid FROM fav.user WHERE uid=$1 AND id>$2 ORDER BY id LIMIT 8192;
  }

  async fn seen_li(uid: u64, ts: u64) -> Result<Vec<(u64, i8, i64)>> {
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
    let li = vb::d(body)?;
    let mut r = VecAny::with_capacity(2);
    r.push(fav_li(uid, li[0]).await?);
    r.push(seen_li(uid, li[1]).await?);
    crate::ws::send(channel_id, WS::同步, r).await?;
    Ok(())
  }
}

pub async fn ws(action: WR, uid: u64, channel_id: String, body: &[u8]) -> Result<()> {
  match action {
    WR::同步 => 同步::run(uid, channel_id, body).await?,
  }
  Ok(())
}
