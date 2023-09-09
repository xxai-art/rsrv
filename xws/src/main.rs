mod header_user;
use std::{fmt::Debug, net::SocketAddr};

use anyhow::Result;
use bytes::BytesMut;
use header_user::header_user;
use lazy_static::lazy_static;
use ratchet_rs::{
  deflate::DeflateExtProvider, CloseReason, Extension, Message, ProtocolRegistry, WebSocket,
  WebSocketConfig, WebSocketResponse,
};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::{wrappers::TcpListenerStream, StreamExt};
use tracing::info;

// https://github.com/Luka967/websocket-close-codes 4000 - 4999   可用于应用
const CODE_UNAUTH: u16 = 4401;

lazy_static! {
  static ref CLOSE_UNAUTH: CloseReason =
    ratchet_rs::CloseReason::new(ratchet_rs::CloseCode::Application(CODE_UNAUTH), None);
}

// sender
//   .close(ratchet_rs::CloseReason::new(
//     ratchet_rs::CloseCode::Application(4401),
//     Some("".to_string()),
//   ))
//   .await?;

async fn close_unauth<T: Extension + Debug>(mut websocket: WebSocket<TcpStream, T>) -> Result<()> {
  websocket.close(CLOSE_UNAUTH.clone()).await?;
  return Ok(());
}

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

  let (mut uri, client_user, mut websocket) = header_user(upgrader).await?;

  if let Some(p) = uri.rfind('/') {
    uri = uri[p + 1..].to_string()
  } else {
    return close_unauth(websocket).await;
  };

  let uid = ub64::b64_u64(uri);
  if !client_user.is_login(uid).await? {
    return close_unauth(websocket).await;
  }

  let mut buf = BytesMut::new();

  let (mut sender, mut receiver) = websocket.split()?;

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
