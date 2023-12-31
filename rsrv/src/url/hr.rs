// hash - w * 1024 / h
use anypack::VecAny;
use awp::anypack::Any;
use axum::body::Bytes;
use intbin::u64_bin;
use x0::{fred::interfaces::HashesInterface, KV, R};
use xc::cid::{CID_IMG, CID_USER};

use crate::K::HR2;

// _client: Client,
pub async fn post(body: Bytes) -> awp::any!() {
  let r: Any;
  if let Some(first) = body.first() {
    match *first {
      b'"' => {
        let t = ub64::b64_decode_u64_li(&body[1..body.len() - 1]);
        let cid = t[0] as u8;
        match cid {
          CID_USER => {
            let result: Vec<Option<String>> = R
              .hmget(
                "userName",
                t[1..]
                  .iter()
                  .map(|i| intbin::u64_bin(*i))
                  .collect::<Vec<_>>(),
              )
              .await?;
            r = result.into();
          }
          _ => {
            r = Any::Null;
          }
        }
      }
      b'[' => {
        let body = String::from_utf8_lossy(&body);
        let input_li: Vec<Vec<u64>> = sonic_rs::from_str(&body)?;

        let mut rli = VecAny::new();
        for li in input_li {
          let cid = li[0] as u8;
          let li = &li[1..];
          let mut tli = anypack::VecAny::new();
          match cid {
            CID_IMG => {
              let key_map: Vec<_> = li.iter().map(|i| u64_bin(*i)).collect();

              let vli: Vec<Option<Vec<u8>>> = KV.hmget(HR2, key_map).await?;
              for v in vli {
                tli.push(v)
              }
            }
            _ => {}
          }
          rli.push(tli)
        }
        r = rli.into();
      }
      _ => {
        r = Any::Null;
      }
    }
  } else {
    r = Any::Null;
  }

  Ok(r)
  //   let FavSync(uid, fav_li) =
  //     sonic_rs::from_str(unsafe { std::str::from_utf8_unchecked(&body) })?;
  //
  //   Ok(if client.is_login(uid).await? {
  //     fav_batch_add(client.id, uid, fav_li).await?
  //   } else {
  //     0
  //   })
}
