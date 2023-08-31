use awp::{any, ok};
use axum::{body::Bytes, http::header::HeaderMap};
use clip_search_txt_client::{clip, DayRange, QIn};
use intbin::u64_bin;
use x0::{fred::interfaces::SortedSetsInterface, KV};
use xxai::{nd::norm01, ndarray::prelude::arr1, time::today};

use crate::{db::img::rec, C::cid::CID_IMG, K};

const IAA_POWER: f32 = 0.6;

pub async fn post(header: HeaderMap, body: Bytes) -> any!() {
  /*
  分级 0 安全 1 不限 2 成人
  */

  if body.is_empty() {
    ok!(rec::li(K::REC0, 0)) // 首页默认背景
  } else {
    let (txt, z85): (String, String) = serde_json::from_str(&String::from_utf8_lossy(&body))?;
    let txt = xxai::str::low_short(txt);
    let z85 = xxai::z85_decode_u64_li(z85)?;
    if txt.is_empty() {
      return ok!(rec::rec(&z85));
    }
    let level = z85[0];
    let key = if level == 2 {
      // 成人
      K::IMG1
    } else if level == 1 {
      // 不限
      K::IMG
    } else {
      // 安全
      K::IMG0
    };
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

    // level 2 成人 1 不限 0 安全
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

    let len = li.len();
    let mut id_li = Vec::with_capacity(len);
    let mut bin_li = Vec::with_capacity(len);
    let mut score_li = Vec::with_capacity(len);
    for i in li {
      id_li.push(i.id);
      bin_li.push(bytes::Bytes::from(u64_bin(i.id)));
      score_li.push(i.score);
    }

    let score_li = norm01(&arr1(&score_li));

    let iaa_li: Vec<Option<f32>> = KV.zmscore(key, bin_li).await?;
    let iaa_li: Vec<_> = iaa_li.into_iter().map(|i| i.unwrap_or(20000.0)).collect();
    let iaa_li = norm01(&arr1(&iaa_li));

    let rank_li = &iaa_li * IAA_POWER + &score_li;

    let mut li = rank_li
      .into_iter()
      .enumerate()
      .map(|(i, n)| (id_li[i], n))
      .collect::<Vec<_>>();

    li.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut r = Vec::with_capacity(len * 2);
    for i in li {
      r.push(CID_IMG);
      r.push(i.0);
    }
    Ok(r.into())
  }
}
