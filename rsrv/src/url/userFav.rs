use anypack::{Any, VecAny};
use awp::any;
use axum::body::Bytes;
use x0::{fred::interfaces::HashesInterface, R};
use intbin::u64_bin;
use xg::Q;

Q!(
li_first_page:
    SELECT cid,rid,ts FROM fav.user WHERE uid=$1 AND aid>0 ORDER BY ts DESC LIMIT 1024;
li:
    SELECT cid,rid,ts FROM fav.user WHERE uid=$1 AND aid>0 AND ts<$2 ORDER BY ts DESC LIMIT 1024
);

fn cid_rid_ts_li(li: Vec<(u16, u64, u64)>) -> VecAny {
  let mut t = VecAny::new();
  if !li.is_empty() {
    let last_ts = li.last().unwrap().2;
    for i in li {
      t.push((i.0, i.1));
    }
    t.push(last_ts);
  }
  t
}

pub async fn post(body: Bytes) -> any!() {
  let (uid, prev_ts): (u64, u64) = serde_json::from_str(&String::from_utf8_lossy(&body))?;
  let name: String = R.hget("userName", u64_bin(uid)).await?;
  Ok(if prev_ts == 0 {
    let mut r = VecAny::new();
    r.push(name);
    r.push(cid_rid_ts_li(li_first_page(uid).await?));
    let r: Any = r.into();
    r
  } else {
    cid_rid_ts_li(li(uid, prev_ts).await?).into()
  })
}
