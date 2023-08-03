use std::{thread, time};

use tokio::time::interval;

static mut TODAY: u64 = 0;

#[ctor::ctor]
fn init() {
  trt::spawn!({
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

      let next = (1 + TODAY) * 86400 + 1;
      interval.set_interval(time::Duration::from_secs(next - now));
    }
  });
}
