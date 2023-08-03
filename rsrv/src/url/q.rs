use awp::{any, ok};
use axum::{body::Bytes, http::header::HeaderMap};
use clip_search_txt_client::{DayRange, OffsetLimit, QIn};
use xxai::time::today;

use crate::db::img::rec;

pub async fn post(header: HeaderMap, body: Bytes) -> any!() {
  /*
  分级 0 安全 1 不限 2 成人
  */

  if body.is_empty() {
    ok!(rec::li())
  } else {
    let (txt, level, duration, end): (String, u64, u64, u64) =
      serde_json::from_str(&String::from_utf8_lossy(&body))?;
    if txt.is_empty() {
      return ok!(rec::li());
    }
    let lang = header
      .get("accept-language")
      .map(|h| h.to_str().unwrap())
      .unwrap_or("en")
      .to_string()
      .into();

    let day_range = None;

    dbg!(today());

    let req = QIn {
      txt: txt.into(),
      nsfw: if level == 1 { -1 } else { level as _ },
      offset_limit: None,
      day_range,
      lang,
    };
    dbg!(&req);
    Ok("".into())
  }

  // let body = &body[1..body.len() - 1];
  // for (pos, i) in body.iter().enumerate() {
  //   if *i == b'/' {
  //     let args = xxai::b64_decode_u64_li(&body[..pos]);
  //     if args.len() >= 3 {
  //       let rating = args[0];
  //       let begin_time = args[1];
  //       let duration = args[2];
  //       let q = String::from_utf8_lossy(&body[1 + pos..]);
  //       if q.is_empty() {
  //         dbg!("TODO 推荐");
  //       } else {
  //         dbg!(rating, begin_time, duration, q);
  //       }
  //     }
  //     break;
  //   }
  // }
}
