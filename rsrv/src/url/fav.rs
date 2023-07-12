use axum::body::Bytes;
use client::Client;
use serde::{Deserialize, Serialize};
use x0::{fred::interfaces::HashesInterface, KV};
use xxpg::{Q, Q01};

use crate::{
  es::{publish_to_user_client, KIND_SYNC_FAV},
  K,
};

// #[derive(Serialize, Debug, Deserialize)]
// struct FavSync(u64, Vec<(u16, u64, u64, i8)>);

Q01!(
    fav_user:
    INSERT INTO fav.user (uid,cid,rid,ts,aid) VALUES ($1,$2,$3,$4,$5) ON CONFLICT (uid, cid, rid, ts) DO NOTHING RETURNING id;

);

Q!(
    fav_rm:
    DELETE FROM fav.user WHERE uid=$1 AND cid=$2 AND rid=$3;
    fav_li:
    SELECT id,cid,rid,ts,aid FROM fav.user WHERE uid=$1 AND id>$2 ORDER BY id;
);

pub async fn fav_batch_add(
  client_id: u64,
  uid: u64,
  fav_li: Vec<(u16, u64, u64, i8)>,
) -> anyhow::Result<u64> {
  let mut id = 0;
  let mut n = 0;
  let mut json = String::new();
  // batch_insert!(
  //   "INSERT INTO fav.user (uid,cid,rid,ts,aid) VALUES {} ON CONFLICT (uid, cid, rid, ts) DO NOTHING RETURNING id",
  //   fav_li.into_iter().map(|x|( uid,x.0,x.1,x.2,x.3 )).collect::<Vec<_>>()
  // );
  for (cid, rid, ts, aid) in fav_li {
    if let Some(_id) = fav_user(uid, cid, rid, ts, aid).await? {
      id = _id;
      n += 1;
      json += &format!("{cid},{rid},{ts},{aid},");
    }
  }
  if n > 0 {
    let p = KV.pipeline();
    p.hincrby(K::FAV_SUM, uid, n).await?;
    p.hset(K::FAV_ID, (uid, id)).await?;
    p.all().await?;
    publish_to_user_client(client_id, uid, KIND_SYNC_FAV, format!("{json}{id}"));
  }
  Ok(id)
}

pub async fn post(client: Client, body: Bytes) -> awp::any!() {
  // let FavSync(uid, fav_li) =
  let mut r = Vec::new();
  let li: Vec<u64> = serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;
  if li.len() > 2 {
    let uid = li[0];
    if client.is_login(uid).await? {
      let last_sync_id = li[1];
      let li: Vec<_> = (&li[2..])
        .chunks_exact(4)
        .map(|i| (i[0] as u16, i[1], i[2], i[3] as i8))
        .collect();

      for i in &li {
        fav_rm(uid, i.0, i.1).await?
      }

      for i in fav_li(uid, last_sync_id).await? {
        r.push(i.1 as u64);
        r.push(i.2);
        r.push(i.3);
        r.push(i.4 as u64);
      }

      // let r = vec![];
      for i in &li {
        // id,cid,rid,ts,aid
      }

      // struct FavSync(u64, Vec<(u16, u64, u64, i8)>);
      for i in li {
        dbg!(uid, last_sync_id, i);
      }

      // TODO select > id 返回，如果没有，就返回last id

      //
      //   fav_batch_add(client.id, uid, fav_li).await?
      // } else {
      //   0
    };
  }
  Ok(r)
}

// macro_rules! batch_insert {
//   ($sql: expr, $li:expr) => {{
//     use xxpg::ToSql;
//
//     let li = $li;
//     let mut args = Vec::<&(dyn ToSql + Sync)>::new();
//     let mut placeholder = String::new();
//     let mut i = 1;
//     // for row in $li {
//     let mut n = 0;
//     let len = li.len();
//     while n < len {
//       let t = &li[n];
//       args.push(&t.0);
//       args.push(&t.1);
//       args.push(&t.2);
//       args.push(&t.3);
//       args.push(&t.4);
//       if n != 0 {
//         placeholder.push(',');
//       }
//
//       placeholder += &format!("(${},${},${},${},${})", i, i + 1, i + 2, i + 3, i + 4);
//       i += 5;
//       n += 1;
//     }
//     let r = xxpg::PG
//       .get()
//       .unwrap()
//       .query(&format!($sql, placeholder), &args)
//       .await;
//     dbg!(&r);
//     if let Ok(li) = r {
//       dbg!(&li);
//       for i in li {
//         dbg!(i.get::<_, Option<u64>>(0));
//       }
//     }
//   }};
// }
