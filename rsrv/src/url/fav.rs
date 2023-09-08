use axum::body::Bytes;
use client::Client;
use xg::{Q, Q01};

use crate::{
  kv::sync::{has_more, set_last},
  ws::send_user,
  C::WS,
  K,
};

// #[derive(Serialize, Debug, Deserialize)]
// struct FavSync(u64, Vec<(u16, u64, u64, i8)>);

Q01!(
fav_user:
    INSERT INTO fav.user (uid,cid,rid,ts,aid) VALUES ($1,$2,$3,$4,$5) ON CONFLICT (uid,cid,rid,ts) DO NOTHING RETURNING id;

);

Q!(
fav_rm:
    DELETE FROM fav.user WHERE uid=$1 AND cid=$2 AND rid=$3;
fav_li:
    SELECT id,cid,rid,ts,aid FROM fav.user WHERE uid=$1 AND id>$2 ORDER BY id;
);

pub fn publish_fav_sync(
  client_id: u64,
  uid: u64,
  prev_id: u64,
  now_id: u64,
  json: impl AsRef<str>,
) {
  let json = json.as_ref();
  send_user(
    uid,
    client_id,
    WS::收藏,
    format!("{prev_id},{now_id}{json}"),
  );
}

pub async fn fav_batch_add(
  prev_id: u64,
  client_id: u64,
  uid: u64,
  fav_li: Vec<(u16, u64, i8)>,
) -> anyhow::Result<u64> {
  let mut id = 0;
  let mut n = 0;
  let mut json = String::new();
  // batch_insert!(
  //   "INSERT INTO fav.user (uid,cid,rid,ts,aid) VALUES {} ON CONFLICT (uid, cid, rid, ts) DO NOTHING RETURNING id",
  //   fav_li.into_iter().map(|x|( uid,x.0,x.1,x.2,x.3 )).collect::<Vec<_>>()
  // );
  let mut ts = sts::ms();
  for (cid, rid, aid) in fav_li {
    if let Some(_id) = fav_user(uid, cid, rid, ts, aid).await? {
      id = _id;
      n += 1;
      json += &format!(",{cid},{rid},{ts},{aid}");
      ts += 1;
    }
  }
  if n > 0 {
    publish_fav_sync(client_id, uid, prev_id, id, json);
  }
  Ok(id)
}

pub async fn post(client: Client, body: Bytes) -> awp::any!() {
  let mut r = Vec::new();
  let li: Vec<u64> = serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;
  if li.len() > 2 {
    let uid = li[0];
    let uid_bin = intbin::u64_bin(uid);

    if client.is_login(uid).await? {
      let last_sync_id = li[1];
      let li: Vec<_> = li[2..]
        .chunks_exact(3)
        .map(|i| (i[0] as u16, i[1], i[2] as i8))
        .collect();

      for i in &li {
        fav_rm(uid, i.0, i.1).await?
      }

      let mut id = 0;

      if has_more(K::FAV_LAST, &uid_bin, last_sync_id).await?.more {
        let fav_li = fav_li(uid, last_sync_id).await?;
        if !fav_li.is_empty() {
          id = fav_li.last().unwrap().0;
          for i in fav_li {
            r.push(i.1 as u64);
            r.push(i.2);
            r.push(i.3);
            r.push(i.4 as u64);
          }
        }
      }

      let last_id = fav_batch_add(last_sync_id, client.id, uid, li).await?;
      if last_id != 0 {
        id = last_id;
      }

      if id != 0 {
        r.push(id);
        set_last(K::FAV_LAST, uid, id);
      }
    };
  }
  Ok(r)
}
