use awp::any;
use axum::body::Bytes;

pub async fn post(body: Bytes) -> any!() {
  let (q, level, duration, end): (String, u64, u64, u64) =
    serde_json::from_str(&String::from_utf8_lossy(&body))?;

  dbg!(q, level, duration, end);
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
  Ok(0)
}
