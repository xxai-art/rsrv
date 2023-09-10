mod header_user;
use std::{net::SocketAddr, sync::Arc};
mod accept;
mod user_ws;
use accept::accept;
use anyhow::Result;
use dashmap::DashMap;
use tokio::net::TcpListener;
use tokio_stream::{wrappers::TcpListenerStream, StreamExt};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
  loginit::init();
  let addr = SocketAddr::from(([0, 0, 0, 0], envport::get("PORT", 8132)));
  info!("ws://{}", addr);

  let listener = TcpListener::bind(addr).await?;
  let mut incoming = TcpListenerStream::new(listener);

  let user_ws = Arc::new(DashMap::new());
  while let Some(Ok(socket)) = incoming.next().await {
    let user_ws = user_ws.clone();
    trt::spawn!({
      accept(user_ws, socket).await?;
    });
  }

  Ok(())
}
