#![feature(const_trait_impl)]

pub mod user;
use axum::{
  http::{Request, StatusCode},
  middleware::Next,
  response::Response,
  Extension,
};
pub use user::_Client;

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
  let (client_user, cookie) = xuser::has_user_client_id_cookie(cookie, host).await;
  req.extensions_mut().insert(_Client(client_user));

  Ok(match cookie {
    Some(cookie) => {
      let mut r = next.run(req).await;
      r.headers_mut()
        .insert(http::header::SET_COOKIE, cookie.parse().unwrap());
      r
    }
    None => next.run(req).await,
  })
}
