use std::time;

use tokio::time::interval;

pub static mut TODAY: u32 = 0;

pub fn today() -> u32 {
  return unsafe { TODAY };
}

pub async fn update_today() {
  loop {
    let now = time::SystemTime::now()
      .duration_since(time::UNIX_EPOCH)
      .unwrap()
      .as_secs();

    let today = now / 86400;
    unsafe {
      TODAY = today as u32;
    }

    let next = (1 + today) * 86400 + 1;

    interval(time::Duration::from_secs(next - now)).tick().await;
  }
}
