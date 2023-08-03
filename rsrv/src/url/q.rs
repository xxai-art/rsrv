use awp::{any, ok};
use axum::{body::Bytes, http::header::HeaderMap};
use clip_search_txt_client::{clip, DayRange, OffsetLimit, QIn};
use xxai::time::today;

use crate::db::img::rec;

pub async fn post(header: HeaderMap, body: Bytes) -> any!() {
  /*
  分级 0 安全 1 不限 2 成人
  */

  if body.is_empty() {
    ok!(rec::img_li())
  } else {
    let (txt, level, duration, end): (String, u64, u32, u32) =
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

    let day_range = if end == 0 && duration == 0 {
      None
    } else {
      let end = if end == 0 { today() + 1 } else { end };
      let begin = end - duration - 1;
      Some(DayRange { begin, end })
    };

    let offset_limit = None; // TODO

    let req = QIn {
      txt: txt.into(),
      nsfw: if level == 1 { -1 } else { level as _ },
      offset_limit,
      day_range,
      lang,
    };
    let li = clip(req).await?.li;
    Ok(li.into_iter().map(|i| i.id).collect::<Vec<_>>().into())
  }
}
