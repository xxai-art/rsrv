mod tld;
use anyhow::Result;
pub use tld::tld;
const COOKIE_SAFE_CHAR: &str =
  "!#$%&'()*+-./0123456789:<>?@ABDEFGHIJKLMNQRSTUVXYZ[]^_`abdefghijklmnqrstuvxyz{|}~";

pub fn cookie_decode(s: &str) -> Result<Box<[u8]>> {
  Ok(base_x::decode(COOKIE_SAFE_CHAR, s)?.into())
}

pub fn cookie_encode(li: Vec<u8>) -> String {
  base_x::encode(COOKIE_SAFE_CHAR, &li)
}

pub fn is_ascii_digit(bytes: &[u8]) -> bool {
  bytes.iter().all(|i| {
    let i = *i;
    i.is_ascii_digit()
  })
}

pub fn zip_u64(li: impl IntoIterator<Item = u64>) -> Result<Vec<u8>> {
  let mut u64_li = vec![];
  for i in li {
    let i: i64 = i.try_into()?;
    u64_li.push(i as u64);
  }
  Ok(vbyte::compress_list(&u64_li))
}

pub fn unzip_u64(bin: impl AsRef<[u8]>) -> Vec<u64> {
  match vbyte::decompress_list(bin.as_ref()) {
    Ok(r) => r,
    Err(_) => vec![],
  }
}
