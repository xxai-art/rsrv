use awp::anypack::Any;
use axum::body::Bytes;
use client::Client;
use x0::{fred::interfaces::HashesInterface, R};

//
// #[derive(Serialize, Debug, Deserialize)]
// struct FavSync(u64, Vec<(u16, u64, u64, i8)>);
//
// Q01!(
// fav_user:
// INSERT INTO fav.user (uid,cid,rid,ts,aid) VALUES ($1,$2,$3,$4,$5) ON CONFLICT (uid, cid, rid, ts) DO NOTHING RETURNING id
// );
//
// pub async fn fav_batch_add(
//   client_id: u64,
//   uid: u64,
//   fav_li: Vec<(u16, u64, u64, i8)>,
// ) -> anyhow::Result<u64> {
//   let mut id = 0;
//   let mut n = 0;
//   let mut json = String::new();
//   // batch_insert!(
//   //   "INSERT INTO fav.user (uid,cid,rid,ts,aid) VALUES {} ON CONFLICT (uid, cid, rid, ts) DO NOTHING RETURNING id",
//   //   fav_li.into_iter().map(|x|( uid,x.0,x.1,x.2,x.3 )).collect::<Vec<_>>()
//   // );
//   for (cid, rid, ts, aid) in fav_li {
//     if let Some(_id) = fav_user(uid, cid, rid, ts, aid).await? {
//       id = _id;
//       n += 1;
//       json += &format!("{cid},{rid},{ts},{aid},");
//     }
//   }
//   if n > 0 {
//     let p = KV.pipeline();
//     p.hincrby(K::FAV_SUM, uid, n).await?;
//     p.hset(K::FAV_ID, (uid, id)).await?;
//     p.all().await?;
//     publish_to_user_client(client_id, uid, KIND_SYNC_FAV, format!("{json}{id}"));
//   }
//   Ok(id)
// }
//
pub async fn post(_client: Client, body: Bytes) -> awp::any!() {
  let r: Any;
  if let Some(first) = body.first() {
    match *first {
      b'"' => {
        let t = xxai::b64_u64_li(&body[1..body.len() - 1]);
        let cid = t[0];
        match cid {
          crate::cid::CID_USER => {
            let result: Vec<Option<String>> = R
              .hmget(
                "userName",
                t[1..].iter().map(|i| xxai::u64_bin(*i)).collect::<Vec<_>>(),
              )
              .await?;
            r = result.into();
          }
          _ => {
            r = Any::Null;
          }
        }
      }
      // b'[' => {}
      _ => {
        r = Any::Null;
      }
    }
  } else {
    r = Any::Null;
  }

  Ok(r)
  //   let FavSync(uid, fav_li) =
  //     serde_json::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;
  //
  //   Ok(if client.is_login(uid).await? {
  //     fav_batch_add(client.id, uid, fav_li).await?
  //   } else {
  //     0
  //   })
}
