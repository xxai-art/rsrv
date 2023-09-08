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

  pub async fn run(uid: u64, channel_id: String, body: &[u8]) -> Result<()> {
    let li = vb::d(body)?;
    let mut seen_ts = li[0];
    let mut fav_id = li[1];
    let _channel_id = channel_id.clone();
    trt::spawn!({
      loop {
        let li = seen_li(uid, seen_ts).await?;
        let len = li.len();
        if len == 0 {
          break;
        }
        let mut r = VecAny::with_capacity(len * 3 + 1);
        let last_ts = li.last().unwrap().0;
        for (ts, cid, rid) in li {
          r.push(ts);
          r.push(cid);
          r.push(rid);
        }
        r.push(seen_ts);
        crate::ws::send(&_channel_id, WS::浏览, r).await?;
        seen_ts = last_ts;
      }
    });
    trt::spawn!({
      loop {
        let li = fav_li(uid, fav_id).await?;
        let len = li.len();
        if len == 0 {
          break;
        }
        let mut r = VecAny::with_capacity(len * 4 + 1);
        let id = li.last().unwrap().0;
        for (_, cid, rid, ts, aid) in li {
          r.push(cid);
          r.push(rid);
          r.push(ts);
          r.push(aid);
        }
        r.push(fav_id);
        crate::ws::send(&channel_id, WS::收藏, r).await?;
        fav_id = id;
      }
    });
    Ok(())
  }
}

pub async fn ws(action: WR, uid: u64, channel_id: String, body: &[u8]) -> Result<()> {
  match action {
    WR::同步 => 同步::run(uid, channel_id, body).await?,
  }
  Ok(())
}
