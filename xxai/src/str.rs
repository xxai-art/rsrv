const MAX_TXT_LEN: usize = 1024;

pub fn low_short(txt: impl AsRef<str>) -> String {
  let txt = txt.as_ref();
  if txt.is_empty() {
    return txt.into();
  }
  if txt.len() > MAX_TXT_LEN {
    txt[..MAX_TXT_LEN].trim()
  } else {
    txt
  }
  .trim()
  .to_lowercase()
}
