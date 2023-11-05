mod user;
use cookie::Cookie;
use gid::gid;
use trt::TRT;
use ub64::bin_u64_li;
pub use user::ClientUser;
use x0::{fred::interfaces::HashesInterface, R};
use xxhash_rust::xxh3::xxh3_64;

static mut SK: [u8; 32] = [0; 32];

pub const MAX_INTERVAL: u64 = 41;
pub const TOKEN_LEN: usize = 8;

/*
   cookie 中的 day 每10天为一个周期，超过41个周期没访问就认为无效, BASE是为了防止数字过大
   https://chromestatus.com/feature/4887741241229312
   When cookies are set with an explicit Expires/Max-Age attribute the value will now be capped to no more than 400 day10s

*/
const BASE: u64 = 4096;

fn day10() -> u64 {
  (xxai::now() / (86400 * 10)) % BASE
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

fn client_by_cookie(cookie: Option<impl AsRef<str>>) -> ClientState {
  if let Some(cookie) = cookie {
    for cookie in Cookie::split_parse(cookie.as_ref()).flatten() {
      if cookie.name() == "I" {
        return client_by_token(cookie.value());
      }
    }
  }
  ClientState::None
}

fn client_by_token(token: &str) -> ClientState {
  if let Ok(c) = xxai::cookie_decode(token) {
    if c.len() >= TOKEN_LEN {
      let client = &c[TOKEN_LEN..];
      if xxh3_64(&[unsafe { &SK }, client].concat())
        == u64::from_le_bytes(c[..TOKEN_LEN].try_into().unwrap())
      {
        let li = bin_u64_li(client);
        if li.len() == 2 {
          let [pre_day10, client]: [u64; 2] = li.try_into().unwrap();

          let now = day10();
          if pre_day10 != now {
            // 因为都是无符号类型，要避免减法出现负数
            if pre_day10 > now {
              if pre_day10 < BASE && (now + BASE - pre_day10) < MAX_INTERVAL {
                return ClientState::Renew(client);
              }
            } else if (now - pre_day10) < MAX_INTERVAL {
              // renew
              return ClientState::Renew(client);
            }
          } else {
            return ClientState::Ok(client);
          }
        }
      }
    }
  }
  ClientState::None
}

gid!(client);

pub async fn client_user_cookie(
  host: impl AsRef<str>,
  cookie: Option<impl AsRef<str>>,
) -> (
  ClientUser,
  Option<String>, // cookie
) {
  let mut client = 0;

  match client_by_cookie(cookie) {
    ClientState::Renew(id) => {
      client = id;
    }
    ClientState::Ok(id) => {
      return (ClientUser { id, _uid: Some(0) }, None);
    }
    _ => {}
  }

  let _uid = if client == 0 {
    client = gid_client().await.unwrap();
    Some(0)
  } else {
    None
  };

  let t = &vbyte::compress_list(&[day10(), client])[..];
  let cookie =
    xxai::cookie_encode([&xxh3_64(&[unsafe { &SK }, t].concat()).to_le_bytes()[..], t].concat());
  let host = xxai::tld(host);
  (
    ClientUser { id: client, _uid },
    Some(format!(
      "I={cookie};max-age=99999999;domain={host};path=/;HttpOnly;SameSite=None;Secure"
    )),
  )
}
