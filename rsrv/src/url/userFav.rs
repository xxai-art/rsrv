use anypack::VecAny;
use awp::{any, ok};
use axum::body::Bytes;
use x0::{fred::interfaces::HashesInterface, KV, R};
use xxai::u64_bin;
use xxpg::Q;

Q!(
    li:
        SELECT cid,rid FROM fav.user WHERE uid=$1 AND aid>0 ORDER BY ts DESC
);

pub async fn post(body: Bytes) -> any!() {
  let uid: u64 = serde_json::from_str(&String::from_utf8_lossy(&body))?;
  let name: String = R.hget("userName", u64_bin(uid)).await?;
  let mut r = VecAny::new();
  r.push(name);
  r.push(li(uid).await?);
  Ok(r)
}
