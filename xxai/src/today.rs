use std::{thread, time};

use tokio::time::interval;

static mut TODAY: u64 = 0;

#[ctor::ctor]
async fn init() {
  let mut interval = interval(time::Duration::from_secs(1));

  loop {
    interval.tick().await;

    let now = time::SystemTime::now()
      .duration_since(time::UNIX_EPOCH)
      .unwrap()
      .as_secs();

    unsafe {
      TODAY = now / 86400;
    }

    let next = TODAY * 86400 + 86400;
    interval.set_interval(time::Duration::from_secs(next - now));
  }
}
