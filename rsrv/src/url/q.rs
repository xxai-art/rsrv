use awp::{any, ok};
use axum::body::Bytes;

pub async fn post(body: Bytes) -> any!() {
  // ok!()
  let q_dl_dt: (String, u8, u8) =
    serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;
  dbg!(q_dl_dt);
  Ok(0)
}
