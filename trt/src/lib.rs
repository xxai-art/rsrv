pub use anyhow;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
pub use tracing;

lazy_static! {
  pub static ref TRT: Runtime = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap();
}

#[macro_export]
macro_rules! spawn {
  ($body:expr) => {{
    tokio::spawn(async move {
      let r: $crate::anyhow::Result<_> = $body.await;
      if let Err(err) = r {
        $crate::tracing::error!("{}", err);
      }
      Ok::<_, trt::anyhow::Error>(())
    })
  }};
}
