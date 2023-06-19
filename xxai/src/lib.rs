mod tld;
use anyhow::Result;
pub use tld::tld;
const COOKIE_SAFE_CHAR: &str =
  "!#$%&'()*+-./0123456789:<>?@ABDEFGHIJKLMNQRSTUVXYZ[]^_`abdefghijklmnqrstuvxyz{|}~";

pub fn now() -> u64 {
  coarsetime::Clock::now_since_epoch().as_secs()
}

pub fn cookie_decode(s: &str) -> Result<Box<[u8]>> {
  Ok(base_x::decode(COOKIE_SAFE_CHAR, s)?.into())
}

pub fn cookie_encode(li: impl AsRef<[u8]>) -> String {
  base_x::encode(COOKIE_SAFE_CHAR, li.as_ref())
}

pub fn is_ascii_digit(bytes: impl AsRef<[u8]>) -> bool {
  bytes.as_ref().iter().all(|i| {
    let i = *i;
    i.is_ascii_digit()
  })
}

pub fn zip_u64(li: impl IntoIterator<Item = u64>) -> Vec<u8> {
  let mut u64_li = vec![];
  for i in li {
    u64_li.push(i as u64);
  }
  vbyte::compress_list(&u64_li)
}

pub fn unzip_u64(bin: impl AsRef<[u8]>) -> Vec<u64> {
  match vbyte::decompress_list(bin.as_ref()) {
    Ok(r) => r,
    Err(_) => vec![],
  }
}

pub fn random_bytes(n: usize) -> Vec<u8> {
  (0..n)
    .map(|_| rand::random::<u8>())
    .collect::<Vec<u8>>()
    .into()
}
