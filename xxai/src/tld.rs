use crate::is_ascii_digit;

pub fn tld(domain: impl AsRef<str>) -> String {
  let domain = domain.as_ref().as_bytes();
  let mut domain = &domain[..];
  if let Some(d) = psl::domain(domain) {
    let bytes = d.suffix().as_bytes();
    let len = bytes.len();
    if len > 0 && !is_ascii_digit(bytes) {
      let mut n = domain.len() - len;
      n = n.saturating_sub(1);
      while n > 0 {
        let t = n - 1;
        if domain[t] == b'.' {
          break;
        }
        n = t;
      }
      domain = &domain[n..]
    }
  }
  unsafe { String::from_utf8_unchecked(domain.into()) }
}
