use std::{fmt::Debug, net::SocketAddr, sync::Arc};

use anyhow::Result;
use bytes::BytesMut;
use dashmap::DashMap;
use lazy_static::lazy_static;
use ratchet_rs::{
  deflate::{DeflateEncoder, DeflateExtProvider},
  CloseReason, Extension, Message, PayloadType, ProtocolRegistry, Sender, WebSocket,
  WebSocketConfig, WebSocketResponse,
};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::{wrappers::TcpListenerStream, StreamExt};
use tracing::info;

use crate::header_user::header_user;

// https://github.com/Luka967/websocket-close-codes 4000 - 4999   可用于应用
const CODE_UNAUTH: u16 = 4401;

// sender
//   .close(ratchet_rs::CloseReason::new(
//     ratchet_rs::CloseCode::Application(4401),
//     Some("".to_string()),
//   ))
//   .await?;

async fn close_unauth<T: Extension + Debug>(
  mut websocket: WebSocket<TcpStream, T>,
  uid: u64,
) -> Result<()> {
  let close_unauth =
    ratchet_rs::CloseReason::new(ratchet_rs::CloseCode::Application(CODE_UNAUTH), None);
  websocket.close(close_unauth).await?;
  return Ok(());
}

pub async fn accept(
  user_ws: Arc<DashMap<u64, Sender<TcpStream, DeflateEncoder>>>,
  socket: TcpStream,
) -> Result<()> {
  let upgrader = ratchet_rs::accept_with(
    socket,
    WebSocketConfig::default(),
    DeflateExtProvider::default(),
    ProtocolRegistry::default(),
  )
  .await?;

  let (mut uri, client_user, mut websocket) = header_user(upgrader).await?;

  if let Some(p) = uri.rfind('/') {
    uri = uri[p + 1..].to_string()
  } else {
    return close_unauth(websocket, 0).await;
  };

  let uid = ub64::b64_u64(uri);
  if !client_user.is_login(uid).await? {
    return close_unauth(websocket, uid).await;
  }

  let mut buf = BytesMut::new();

  let (mut sender, mut receiver) = websocket.split()?;

  user_ws.insert(uid, sender);
  loop {
    match receiver.read(&mut buf).await? {
      Message::Text => {
        //dbg!("txt", &buf);
        // sender.write(&mut buf, PayloadType::Text).await?;
        //buf.clear();
      }
      Message::Binary => {
        dbg!("bin", &buf);
        // sender.write(&mut buf, PayloadType::Binary).await?;
        buf.clear();
      }
      Message::Ping(_) | Message::Pong(_) => {
        //dbg!("ping pong");
      }
      Message::Close(_) => break,
    }
  }
  Ok(())
}
