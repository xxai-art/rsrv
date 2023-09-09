use std::fmt::Debug;

use anyhow::Result;
use ratchet_rs::{
  deflate::Deflate, Extension, HeaderMap, UpgradedServer, WebSocket, WebSocketUpgrader,
};
use tokio::net::TcpStream;
use ub64::b64_u64;

pub async fn header_user<T: Extension + Debug>(
  upgrader: WebSocketUpgrader<TcpStream, T>,
) -> Result<Option<WebSocket<TcpStream, T>>> {
  let req = upgrader.request();
  let uri = {
    let uri = req.uri().to_string();
    if let Some(p) = uri.rfind('/') {
      uri[p + 1..].to_string()
    } else {
      return Ok(None);
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
    return Ok(None);
  }

  let uid = b64_u64(uri);

  let mut headers = HeaderMap::new();
  headers.insert("xxx", "abc".parse()?);

  // let response = WebSocketResponse::with_headers(200, headers);
  let UpgradedServer { websocket, .. } = upgrader.upgrade_with(headers).await?;
  // let UpgradedServer { websocket, .. } = upgrader.upgrade().await?;
  Ok(Some(websocket))
}
