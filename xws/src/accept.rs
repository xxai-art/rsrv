use std::{fmt::Debug, sync::Arc};

use anyhow::Result;
use bytes::BytesMut;
use dashmap::DashMap;
use int_enum::IntEnum;
use ratchet_rs::{
  deflate::DeflateExtProvider, Extension, Message, PayloadType, ProtocolRegistry, WebSocket,
  WebSocketConfig,
};
use tokio::{net::TcpStream, sync::Mutex};

use crate::{header_user::header_user, r#type::AllWs, recv::recv, C::RECV};

// https://github.com/Luka967/websocket-close-codes 4000 - 4999   可用于应用
const CODE_UNAUTH: u16 = 4401;

async fn close_unauth<T: Extension + Debug>(mut websocket: WebSocket<TcpStream, T>) -> Result<()> {
  let close_unauth =
    ratchet_rs::CloseReason::new(ratchet_rs::CloseCode::Application(CODE_UNAUTH), None);
  websocket.close(close_unauth).await?;
  Ok(())
}

pub async fn accept(user_ws: AllWs, socket: TcpStream) -> Result<()> {
  let upgrader = ratchet_rs::accept_with(
    socket,
    WebSocketConfig::default(),
    DeflateExtProvider::default(),
    ProtocolRegistry::default(),
  )
  .await?;

  let (mut uri, client_user, websocket) = header_user(upgrader).await?;

  if let Some(p) = uri.rfind('/') {
    uri = uri[p + 1..].to_string()
  } else {
    return close_unauth(websocket).await;
  };

  let uid = ub64::b64_u64(uri);
  if !client_user.is_login(uid).await? {
    return close_unauth(websocket).await;
  }

  let client_id = client_user.id;

  let (sender, mut receiver) = websocket.split()?;

  let sender = Arc::new(Mutex::new(sender));

  user_ws
    .entry(uid)
    .or_default()
    .insert(client_id, sender.clone());

  let mut buf = BytesMut::new();

  loop {
    match receiver.read(&mut buf).await? {
      Message::Text => {
        //dbg!("txt", &buf);
        // sender.write(&mut buf, PayloadType::Text).await?;
        //buf.clear();
      }
      Message::Binary => {
        if !buf.is_empty() {
          if let Ok(kind) = RECV::from_int(buf[0]) {
            match recv(kind, &buf[1..], || user_ws.clone()).await {
              Ok(bin) => {
                if let Some(bin) = bin {
                  sender.lock().await.write(&bin, PayloadType::Binary).await?;
                }
              }
              Err(err) => {
                tracing::error!("{} {}", uid, err)
              }
            }
          }
        }
      }
      Message::Ping(_) => {
        sender.lock().await.write(&[], PayloadType::Pong).await?;
      }
      Message::Pong(_) => {
        // todo 超时断开
      }
      Message::Close(_) => break,
    }
    buf.clear();
  }
  if let Some(map) = user_ws.get(&uid) {
    if map.len() == 1 {
      user_ws.remove(&uid);
    } else {
      map.remove(&client_id);
    }
  };
  Ok(())
}
