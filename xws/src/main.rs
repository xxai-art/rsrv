use std::net::SocketAddr;

use anyhow::Result;
use bytes::BytesMut;
use ratchet_rs::{
  deflate::DeflateExtProvider, Error, HeaderMap, Message, PayloadType, ProtocolRegistry,
  UpgradedServer, WebSocketConfig, WebSocketResponse,
};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::{wrappers::TcpListenerStream, StreamExt};
use tracing::info;
use ub64::b64_u64;

async fn verify(req: &http::request::Request<()>) -> Result<bool> {
  let uri = {
    let uri = req.uri().to_string();
    if let Some(p) = uri.rfind('/') {
      uri[p + 1..].to_string()
    } else {
      return Ok(false);
    }
  };

  let mut cookie_i = String::new();
  if let Some(cookie) = req.headers().get("cookie") {
    for i in cookie.to_str()?.split(';') {
      if i.starts_with("I=") {
        cookie_i = i[2..].trim().to_string();
      }
    }
  };
  if cookie_i.is_empty() {
    return Ok(false);
  }

  let uid = b64_u64(uri);
  Ok(true)
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
  // websocket.reject(WebSocketResponse::new(404)?).await?;

  // Or you could opt to reject the connection with headers
  // websocket.reject(WebSocketResponse::with_headers(404, headers)?).await;

  if !verify(upgrader.request()).await? {
    return Ok(());
  }

  let mut headers = HeaderMap::new();
  headers.insert("xxx", "abc".parse()?);

  // let response = WebSocketResponse::with_headers(200, headers);
  let UpgradedServer { websocket, .. } = upgrader.upgrade_with(headers).await?;
  // let UpgradedServer { websocket, .. } = upgrader.upgrade().await?;
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
