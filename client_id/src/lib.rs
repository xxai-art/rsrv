use axum::{
  extract::{Extension, Host},
  http::{header::COOKIE, Request, StatusCode},
  middleware::{self, Next},
  response::{IntoResponse, Response},
  routing::get,
  Router,
};
// use tower_cookies::Cookies;
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
    let redis = R.force().await;
    let conf = &b"conf"[..];
    let key = &b"SK"[..];
    let sk: Option<Vec<u8>> = redis.hget(conf, key).await.unwrap();
    let len = unsafe { SK.len() };
    if let Some(sk) = sk {
      if sk.len() == len {
        dbg!(">", &sk);
        unsafe { SK = sk.try_into().unwrap() };
        return;
      }
    }
    use xxai::random_bytes;
    let sk = &random_bytes(len)[..];
    redis.hset::<(), _, _>(conf, vec![(key, sk)]).await.unwrap();
    dbg!(&sk);
    unsafe { SK = sk.try_into().unwrap() };
  })
}

// fn client_id_by_cookie(Host(host): Host, cookies: &Cookies) -> u64 {
//   use tower_cookies::{
//     cookie::{time::Duration, SameSite},
//     Cookie,
//   };
//
//   dbg!(tld(&host));
//
//   cookies.add(
//     Cookie::build("hello_world_key", "hello_world_val4")
//       .max_age(Duration::seconds(99999999))
//       .secure(true)
//       .path("/")
//       .domain(tld(&host))
//       .same_site(SameSite::None)
//       .http_only(true)
//       .finish(),
//   );
//   if let Some(c) = cookies.get("I") {
//     if let Ok(c) = xxai::cookie_decode(c.value()) {
//       if c.len() >= TOKEN_LEN {
//         let client = &c[TOKEN_LEN..];
//         if xxh3_64(&[unsafe { &SK }, client].concat())
//           == u64::from_le_bytes(c[..TOKEN_LEN].try_into().unwrap())
//         {
//           let li = unzip_u64(client);
//           if li.len() == 2 {
//             let [day, client_id]: [u64; 2] = li.try_into().unwrap();
//
//             /*
//              每10天为一个周期，超过40个周期没访问就认为无效, BASE是为了防止数字过大
//              https://chromestatus.com/feature/4887741241229312
//              When cookies are set with an explicit Expires/Max-Age attribute the value will now be capped to no more than 400 days
//             */
//
//             let now = (xxai::now() / 864000) % BASE;
//             dbg!(day, client_id, now);
//             if day != now {
//               if ((now - day) < MAX_INTERVAL) || (day > now && (now + BASE - day) < MAX_INTERVAL) {
//                 // renew
//                 return client_id;
//               }
//             } else {
//               return client_id;
//             }
//           }
//         }
//       }
//     }
//   }
//   // client_id
//   0
// }

#[derive(Clone)]
struct ClientId {
  id: u64,
}

fn header_get<'a, B>(req: &'a Request<B>, key: impl AsRef<str>) -> Option<&'a str> {
  req
    .headers()
    .get(key.as_ref())
    .and_then(|header| header.to_str().ok())
}

pub async fn client_id<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
  if let Some(cookie) = header_get(&req, http::header::COOKIE) {}
  let host = xxai::tld(header_get(&req, http::header::HOST).unwrap());
  // let cookie = header
  //   .get(http::header::COOKIE)
  //   .and_then(|header| header.to_str().ok());

  dbg!(&host);
  //
  // let auth_header = if let Some(auth_header) = auth_header {
  //   auth_header
  // } else {
  //   return Err(StatusCode::UNAUTHORIZED);
  // };

  // if let Some(current_user) = authorize_current_user(auth_header).await {
  //   // insert the current user into a request extension so the handler can
  //   // extract it
  //   req.extensions_mut().insert(current_user);
  //   Ok(next.run(req).await)
  // } else {
  //   Err(StatusCode::UNAUTHORIZED)
  // }

  let mut r = next.run(req).await;
  let client_id = 1;
  let t = &xxai::zip_u64([day(), client_id])[..];
  let cookie = xxai::cookie_encode(xxh3_64(&[unsafe { &SK }, t].concat()).to_le_bytes());
  //         if unsafe { &SK }, client].concat())
  // sk = xxh64(SK,args)
  // cookie = cookieEncode(sk, args)

  r.headers_mut().insert(
    http::header::SET_COOKIE,
    format!("I={cookie};max-age=99999999;domain={host};path=/;HttpOnly;SameSite=None;Secure")
      .parse()
      .unwrap(),
  );
  dbg!(&r.headers());
  Ok(r)
}
