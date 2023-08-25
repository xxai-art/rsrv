mod diffli;
pub mod str;
pub mod time;
mod tld;
// mod vbyte_decode;

#[cfg(feature = "ndarray")]
pub mod nd;
use anyhow::Result;
#[cfg(feature = "ndarray")]
pub use ndarray;
pub use tld::tld;

pub use crate::diffli::diffli;

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

pub fn z85_decode_u64_li(s: impl AsRef<str>) -> Result<Vec<u64>> {
  Ok(vb::d(z85::decode(s.as_ref())?)?)
}

pub fn z85_encode_u64_li(u64_li: Vec<u64>) -> String {
  z85::encode(vb::e(&u64_li))
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
