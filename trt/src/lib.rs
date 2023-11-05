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
      if let Err(err) = $body.await {
        $crate::tracing::error!("{}", err);
      }
      Ok::<_, $crate::anyhow::Error>(())
    })
  }};
}
