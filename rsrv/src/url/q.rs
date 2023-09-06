use awp::{any, ok};
use axum::{body::Bytes, http::header::HeaderMap};
use clip_search_txt_client::{clip, DayRange, Level, QIn};
use xxai::time::today;

use crate::{
  db::{img::rec, score::sort},
  C::{cid::CID_IMG, 分级},
  K,
};

pub async fn post(header: HeaderMap, body: Bytes) -> any!() {
  if body.is_empty() {
    ok!(rec::li(K::R1, 0)) // 首页默认背景
  } else {
    let (txt, z85): (String, String) = serde_json::from_str(&String::from_utf8_lossy(&body))?;
    let txt = xxai::str::low_short(txt);
    let z85 = xxai::z85_decode_u64_li(z85)?;
    if txt.is_empty() {
      return ok!(rec::rec(&z85));
    }
    let level = z85[0] as u8;
    let duration = z85[1] as u32;
    let end = z85[2] as u32;
    // let w = z85[3];
    // let h = z85[4];
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
      // w: w as u32,
      // h: h as u32,
      txt: txt.into(),
      level: match level {
        分级::不限 => Level::All,
        分级::成人 => Level::Nsfw,
        _ => Level::Sfw,
      },
      offset_limit,
      day_range,
      lang,
    };
    let li = clip(req).await?.li;

    let len = li.len();
    let mut id_li = Vec::with_capacity(len);
    let mut score_li = Vec::with_capacity(len);
    let mut quality_li = Vec::with_capacity(len);
    for i in li {
      id_li.push(i.id);
      score_li.push(i.score);
      quality_li.push(i.quality as f32);
    }

    let li = sort(id_li, score_li, quality_li);
    let mut r = Vec::with_capacity(len * 2);
    for i in li {
      r.push(CID_IMG as u64);
      r.push(i);
    }
    Ok(r.into())
  }
}
