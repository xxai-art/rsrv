pub use anyhow;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
pub use tokio::spawn;
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
    $crate::spawn(async move {
      if let Err(err) = $body.await {
        $crate::tracing::error!("{}", err);
      }
      Ok::<_, $crate::anyhow::Error>(())
    })
  }};
}
