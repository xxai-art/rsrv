pub mod time;
mod tld;
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
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

// pub fn zip_u64(li: impl IntoIterator<Item = u64>) -> Vec<u8> {
//   let mut u64_li = vec![];
//   for i in li {
//     u64_li.push(i);
//   }
//   vbyte::compress_list(&u64_li)
// }

pub fn b64_u64_li(bin: impl AsRef<[u8]>) -> Vec<u64> {
  let bin = bin.as_ref();
  if let Ok(r) = URL_SAFE_NO_PAD.decode(bin) {
    return bin_u64_li(r);
  }
  vec![]
}

pub fn bin_u64_li(bin: impl AsRef<[u8]>) -> Vec<u64> {
  match vbyte::decompress_list(bin.as_ref()) {
    Ok(r) => r,
    Err(_) => vec![],
  }
}

pub fn random_bytes(n: usize) -> Vec<u8> {
  (0..n).map(|_| rand::random::<u8>()).collect::<Vec<u8>>()
}

pub fn u64_bin_ordered(n: u64) -> Vec<u8> {
  use ordered_varint::Variable;
  n.to_variable_vec().unwrap()
}

pub fn ordered_bin_u64(bin: impl AsRef<[u8]>) -> u64 {
  use ordered_varint::Variable;
  u64::decode_variable(bin.as_ref()).unwrap()
}

pub fn b64_u64(bin: impl AsRef<[u8]>) -> u64 {
  if let Ok(r) = URL_SAFE_NO_PAD.decode(bin.as_ref()) {
    return bin_u64(r);
  }
  0
}

pub fn bin_u64(bin: impl AsRef<[u8]>) -> u64 {
  let bin = bin.as_ref();
  let mut b = [0u8; 8];
  b[..bin.len()].copy_from_slice(bin);
  return u64::from_le_bytes(b);
}

pub fn u64_bin(n: u64) -> Box<[u8]> {
  let n = n.to_le_bytes();
  let mut i = 8;
  while i > 0 {
    let p = i - 1;
    if n[p] != 0 {
      break;
    }
    i = p;
  }
  Box::from(&n[..i])
}

pub fn b64(bin: impl AsRef<[u8]>) -> String {
  URL_SAFE_NO_PAD.encode(bin)
}

pub fn u64_b64(n: u64) -> String {
  URL_SAFE_NO_PAD.encode(&u64_bin(n))
}
