#![feature(const_trait_impl)]

pub mod user;
use axum::{
  http::{Request, StatusCode},
  middleware::Next,
  response::Response,
  Extension,
};

#[derive(Debug, Clone, Copy)]
pub struct _Client {
  pub id: u64,
  _uid: Option<u64>,
}

pub type Client = Extension<_Client>;

fn header_get<B>(req: &Request<B>, key: impl AsRef<str>) -> Option<&str> {
  req
    .headers()
    .get(key.as_ref())
    .and_then(|header| header.to_str().ok())
}

pub async fn client<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
  _client(req, next).await
}

pub async fn _client<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
  let cookie = header_get(&req, http::header::COOKIE);
  let host = header_get(&req, http::header::HOST).unwrap();
  let (has_user, client_id, cookie) = xuser::has_user_client_id_cookie(cookie, host).await;

  Ok(match cookie {
    Some(cookie) => {
      req.extensions_mut().insert(_Client {
        id: client_id,
        _uid: if has_user { Some(0) } else { None },
      });
      let mut r = next.run(req).await;
      r.headers_mut()
        .insert(http::header::SET_COOKIE, cookie.parse().unwrap());

      r
    }
    None => {
      req.extensions_mut().insert(_Client {
        id: client_id,
        _uid: Some(0),
      });
      next.run(req).await
    }
  })
  // let mut client = 0;
  //
  // match xuser::client_by_cookie(cookie) {
  //   ClientState::Renew(id) => {
  //     client = id;
  //   }
  //   ClientState::Ok(id) => {
  //     req.extensions_mut().insert(_Client { id, _uid: Some(0) });
  //     return Ok(next.run(req).await);
  //   }
  //   _ => {}
  // }
  //
  // let _uid = if client == 0 {
  //   client = gid!(client);
  //   Some(0)
  // } else {
  //   None
  // };
  // req.extensions_mut().insert(_Client { id: client, _uid });
  // let mut r = next.run(req).await;
  //
  // let t = &vbyte::compress_list(&[day10(), client])[..];
  // let cookie =
  //   xxai::cookie_encode([&xxh3_64(&[unsafe { &SK }, t].concat()).to_le_bytes()[..], t].concat());
  // let host = xxai::tld(host);
  // r.headers_mut().insert(
  //   http::header::SET_COOKIE,
  //   format!("I={cookie};max-age=99999999;domain={host};path=/;HttpOnly;SameSite=None;Secure")
  //     .parse()
  //     .unwrap(),
  // );
}
