use axum::{
  http::{Request, StatusCode},
  middleware::Next,
  response::Response,
};
use cookie::Cookie;
use gid::gid;
use trt::TRT;
use x0::{fred::interfaces::HashesInterface, R};
use xxai::unzip_u64;
use xxhash_rust::xxh3::xxh3_64;

static mut SK: [u8; 32] = [0; 32];

const MAX_INTERVAL: u64 = 41;
const BASE: u64 = 4096;

const TOKEN_LEN: usize = 8;

fn day() -> u64 {
  (xxai::now() / 864000) % BASE
}

#[ctor::ctor]
fn init() {
  TRT.block_on(async move {
    let redis = R.0.force().await;
    let conf = &b"conf"[..];
    let key = &b"SK"[..];
    let sk: Option<Vec<u8>> = redis.hget(conf, key).await.unwrap();
    let len = unsafe { SK.len() };
    if let Some(sk) = sk {
      if sk.len() == len {
        unsafe { SK = sk.try_into().unwrap() };
        return;
      }
    }
    use xxai::random_bytes;
    let sk = &random_bytes(len)[..];
    redis.hset::<(), _, _>(conf, vec![(key, sk)]).await.unwrap();
    unsafe { SK = sk.try_into().unwrap() };
  })
}

#[derive(Debug)]
pub enum ClientState {
  Ok(u64),
  Renew(u64),
  None,
}

fn client_id_by_cookie(token: &str) -> ClientState {
  if let Ok(c) = xxai::cookie_decode(token) {
    if c.len() >= TOKEN_LEN {
      let client = &c[TOKEN_LEN..];
      if xxh3_64(&[unsafe { &SK }, client].concat())
        == u64::from_le_bytes(c[..TOKEN_LEN].try_into().unwrap())
      {
        let li = unzip_u64(client);
        if li.len() == 2 {
          let [day, client_id]: [u64; 2] = li.try_into().unwrap();

          /*
          每10天为一个周期，超过40个周期没访问就认为无效, BASE是为了防止数字过大
          https://chromestatus.com/feature/4887741241229312
          When cookies are set with an explicit Expires/Max-Age attribute the value will now be capped to no more than 400 days
          */

          let now = (xxai::now() / 864000) % BASE;
          if day != now {
            // 因为都是无符号类型，要避免减法出现负数
            if day > now {
              if day < BASE && (now + BASE - day) < MAX_INTERVAL {
                return ClientState::Renew(client_id);
              }
            } else if (now - day) < MAX_INTERVAL {
              // renew
              return ClientState::Renew(client_id);
            }
          } else {
            return ClientState::Ok(client_id);
          }
        }
      }
    }
  }
  ClientState::None
}

#[derive(Debug, Clone, Copy)]
pub struct Client {
  pub id: u64,
}

fn header_get<B>(req: &Request<B>, key: impl AsRef<str>) -> Option<&str> {
  req
    .headers()
    .get(key.as_ref())
    .and_then(|header| header.to_str().ok())
}

pub async fn client_id<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
  let mut client_id = 0;

  if let Some(cookie) = header_get(&req, http::header::COOKIE) {
    for cookie in Cookie::split_parse(cookie).flatten() {
      if cookie.name() == "I" {
        match client_id_by_cookie(cookie.value()) {
          ClientState::Renew(id) => {
            dbg!("renew", id);
            client_id = id;
          }
          ClientState::Ok(id) => {
            dbg!("ok", id);
            req.extensions_mut().insert(Client { id });
            return Ok(next.run(req).await);
          }
          _ => {}
        }
        break;
      }
    }
  }

  let host = xxai::tld(header_get(&req, http::header::HOST).unwrap());
  if client_id == 0 {
    client_id = gid!(client);
  }
  dbg!("set cookie", client_id);
  req.extensions_mut().insert(Client { id: client_id });

  let mut r = next.run(req).await;

  let t = &xxai::zip_u64([day(), client_id])[..];
  let cookie =
    xxai::cookie_encode([&xxh3_64(&[unsafe { &SK }, t].concat()).to_le_bytes()[..], t].concat());
  r.headers_mut().insert(
    http::header::SET_COOKIE,
    format!("I={cookie};max-age=99999999;domain={host};path=/;HttpOnly;SameSite=None;Secure")
      .parse()
      .unwrap(),
  );
  Ok(r)
}
