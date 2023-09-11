#[allow(non_snake_case)]
mod C;
mod db;
mod header_user;
mod recv;
use std::net::SocketAddr;
mod accept;
mod r#type;
use accept::accept;
use anyhow::Result;
use tokio::net::TcpListener;
use tokio_stream::{wrappers::TcpListenerStream, StreamExt};
use tracing::info;

use crate::r#type::AllWs;

#[tokio::main]
async fn main() -> Result<()> {
  loginit::init();
  let addr = SocketAddr::from(([0, 0, 0, 0], envport::get("PORT", 8133)));
  info!("ws://{}", addr);

  let listener = TcpListener::bind(addr).await?;
  let mut incoming = TcpListenerStream::new(listener);

  let all_ws = AllWs::default();
  while let Some(Ok(socket)) = incoming.next().await {
    let all_ws = all_ws.clone();
    trt::spawn!({
      accept(all_ws, socket).await?;
    });
  }

  Ok(())
}
