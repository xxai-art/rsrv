use anyhow::Result;

const COOKIE_SAFE_CHAR: &str =
  "!#$%&'()*+-./0123456789:<>?@ABDEFGHIJKLMNQRSTUVXYZ[]^_`abdefghijklmnqrstuvxyz{|}~";

pub fn cookie_decode(s: &str) -> Result<Box<[u8]>> {
  Ok(base_x::decode(COOKIE_SAFE_CHAR, s)?.into())
}

pub fn cookie_encode(li: Vec<u8>) -> String {
  base_x::encode(COOKIE_SAFE_CHAR, &li)
}
