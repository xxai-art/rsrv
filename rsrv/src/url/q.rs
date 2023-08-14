use awp::{any, ok};
use axum::{body::Bytes, http::header::HeaderMap};
use clip_search_txt_client::{clip, DayRange, OffsetLimit, QIn};
use x0::{fred::interfaces::HashesInterface, KV};
use xxai::{bin_u64, time::today, u64_bin};

use crate::{cid::CID_IMG, db::img::rec};

pub async fn post(header: HeaderMap, body: Bytes) -> any!() {
  /*
  分级 0 安全 1 不限 2 成人
  */

  if body.is_empty() {
    ok!(rec::img_li())
  } else {
    let (txt, z85): (String, String) = serde_json::from_str(&String::from_utf8_lossy(&body))?;
    if txt.is_empty() {
      return ok!(rec::li());
    }
    let z85 = xxai::z85_decode_u64_li(z85)?;
    let level = z85[0];
    let duration = z85[1] as u32;
    let end = z85[2] as u32;
    let w = z85[3];
    let h = z85[4];
    let lang = header
      .get("accept-language")
      .map(|h| h.to_str().unwrap())
      .unwrap_or("en")
      .to_string()
      .into();

    let day_range = if end == 0 && duration == 0 {
      None
    } else {
      let end = if end == 0 { today() + 1 } else { end };
      let begin = end - duration - 1;
      Some(DayRange { begin, end })
    };

    let offset_limit = None; // TODO

    let req = QIn {
      w: w as u32,
      h: h as u32,
      txt: txt.into(),
      nsfw: if level == 1 { -1 } else { level as _ },
      offset_limit,
      day_range,
      lang,
    };
    let li = clip(req).await?.li;
    let id_li: Vec<_> = li.iter().map(|i| u64_bin(i.id)).collect();
    let score_li: Vec<Bytes> = KV.hmget("iaa", id_li).await?;
    let score_li: Vec<_> = score_li
      .into_iter()
      .map(|i| {
        let i = bin_u64(i);
        if i > 128 {
          26.0
        } else {
          i as f64
        }
      })
      .collect();
    dbg!(li.len());

    let mut r = Vec::with_capacity(li.len() * 2);
    for i in li {
      r.push(CID_IMG);
      r.push(i.id);
    }
    Ok(r.into())
  }
}
