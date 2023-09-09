use std::net::SocketAddr;

use anyhow::Result;
use bytes::BytesMut;
use ratchet_rs::{
  deflate::DeflateExtProvider, Error, Message, PayloadType, ProtocolRegistry, UpgradedServer,
  WebSocketConfig,
};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::{wrappers::TcpListenerStream, StreamExt};
use tracing::info;

async fn accpet(socket: TcpStream) -> Result<()> {
  let upgrader = ratchet_rs::accept_with(
    socket,
    WebSocketConfig::default(),
    DeflateExtProvider::default(),
    ProtocolRegistry::default(),
  )
  .await?;

  // You could opt to reject the connection
  // websocket.reject(WebSocketResponse::new(404)?).await?;

  // Or you could opt to reject the connection with headers
  // websocket.reject(WebSocketResponse::with_headers(404, headers)?).await;

  let req = &upgrader.request();
  let uri = {
    let uri = req.uri().to_string();
    if let Some(p) = uri.rfind('/') {
      uri[p + 1..].to_string()
    } else {
      return Ok(());
    }
  };

  if let Some(cookie) = req.headers().get("cookie") {
    dbg!(cookie);
  }

  dbg!(uri);

  let UpgradedServer { websocket, .. } = upgrader.upgrade().await?;
  let mut buf = BytesMut::new();

  let (mut sender, mut receiver) = websocket.split()?;

  loop {
    match receiver.read(&mut buf).await? {
      Message::Text => {
        dbg!("txt", &buf);
        sender.write(&mut buf, PayloadType::Text).await?;
        buf.clear();
      }
      Message::Binary => {
        dbg!("bin", &buf);
        sender.write(&mut buf, PayloadType::Binary).await?;
        buf.clear();
      }
      Message::Ping(_) | Message::Pong(_) => {
        dbg!("ping pong");
      }
      Message::Close(_) => break,
    }
  }
  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  loginit::init();
  let addr = SocketAddr::from(([0, 0, 0, 0], envport::get("PORT", 8132)));
  info!("ws://{}", addr);

  let listener = TcpListener::bind(addr).await?;
  let mut incoming = TcpListenerStream::new(listener);

  while let Some(Ok(socket)) = incoming.next().await {
    trt::spawn!({
      accpet(socket).await?;
    });
  }

  Ok(())
}
