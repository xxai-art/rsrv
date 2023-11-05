use std::{fmt::Debug, sync::Arc};

use anyhow::Result;
use bytes::BytesMut;
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

pub async fn accept(all_ws: AllWs, socket: TcpStream) -> Result<()> {
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

  all_ws.insert(uid, client_id, sender.clone());

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
            let all_ws = all_ws.clone();
            let msg = Box::from(&buf[1..]);
            trt::spawn!(async move {
              if let Err(err) = recv(kind, &msg, uid, client_id, all_ws).await {
                tracing::error!("{} {}", uid, err)
              }
              Ok::<_, anyhow::Error>(())
            });
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
  all_ws.remove(uid, client_id);
  Ok(())
}
