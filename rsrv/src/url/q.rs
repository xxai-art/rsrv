use awp::{any, ok};
use axum::{body::Bytes, http::header::HeaderMap};
use clip_search_txt_client::{clip, DayRange, QIn};
use x0::{fred::interfaces::HashesInterface, KV};
use xxai::{bin_u64, nd::norm01, ndarray::prelude::arr1, time::today, u64_bin};

use crate::{cid::CID_IMG, db::img::rec};

const IAA_POWER: f32 = 0.5;
const MAX_TXT_LEN: usize = 2048;
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
    let txt = if txt.len() > MAX_TXT_LEN {
      txt[..MAX_TXT_LEN].to_string()
    } else {
      txt
    }
    .to_lowercase();
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

    let len = li.len();
    let mut id_li = Vec::with_capacity(len);
    let mut bin_li = Vec::with_capacity(len);
    let mut score_li = Vec::with_capacity(len);
    for i in li {
      id_li.push(i.id);
      bin_li.push(u64_bin(i.id));
      score_li.push(i.score);
    }

    let score_li = norm01(&arr1(&score_li));

    let iaa_li: Vec<Bytes> = KV.hmget("iaa", bin_li).await?;
    let iaa_li: Vec<_> = iaa_li
      .into_iter()
      .map(|i| {
        let i = bin_u64(i);
        if i > 128 {
          //开发服务器KV打分未必完整, None 会变成特别大的数字
          26.0
        } else {
          i as f32
        }
      })
      .collect();
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
