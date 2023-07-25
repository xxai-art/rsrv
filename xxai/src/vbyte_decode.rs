use thiserror::Error;
use vbyte::decompress_list;

#[derive(Error, Debug)]
pub enum Error {
  #[error("vbyte decode: {0}")]
  VbyteDecode(String),
}

pub fn vbyte_decode(bin: impl AsRef<[u8]>) -> Result<Vec<u64>, Error> {
  match decompress_list(bin.as_ref()) {
    Ok(r) => Ok(r),
    Err(err) => Err(Error::VbyteDecode(err.to_string())),
  }
}
