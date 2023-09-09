mod header_user;
use std::net::SocketAddr;

use anyhow::Result;
use bytes::BytesMut;
use header_user::header_user;
use ratchet_rs::{
  deflate::DeflateExtProvider, Message, ProtocolRegistry, WebSocketConfig, WebSocketResponse,
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
  // let response = WebSocketResponse::with_headers(200, headers);
  // websocket.reject(WebSocketResponse::new(404)?).await?;

  // Or you could opt to reject the connection with headers
  // websocket.reject(WebSocketResponse::with_headers(404, headers)?).await;

  let (mut uri, client_user, websocket) = header_user(upgrader).await?;

  if let Some(p) = uri.rfind('/') {
    uri = uri[p + 1..].to_string()
  };
  let uid = ub64::b64_u64(uri);
  dbg!(uid);
  // if websocket.is_none() {
  //   return Ok(());
  // }
  // let websocket = websocket.unwrap();

  let mut buf = BytesMut::new();

  let (mut sender, mut receiver) = websocket.split()?;

  // https://github.com/Luka967/websocket-close-codes 4000 - 4999   可用于应用
  // sender
  //   .close(ratchet_rs::CloseReason::new(
  //     ratchet_rs::CloseCode::Application(4401),
  //     Some("".to_string()),
  //   ))
  //   .await?;

  loop {
    match receiver.read(&mut buf).await? {
      Message::Text => {
        dbg!("txt", &buf);
        // sender.write(&mut buf, PayloadType::Text).await?;
        buf.clear();
      }
      Message::Binary => {
        dbg!("bin", &buf);
        // sender.write(&mut buf, PayloadType::Binary).await?;
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
