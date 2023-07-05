use lazy_static::lazy_static;
use tokio::runtime::Runtime;

lazy_static! {
  pub static ref TRT: Runtime = Runtime::new().unwrap();
}

#[macro_export]
macro_rules! spawn {
  ($body:stmt) => {
    tokio::spawn(async move {
      $body.await?;
      Ok::<_, anyhow::Error>(())
    })
  };
}
