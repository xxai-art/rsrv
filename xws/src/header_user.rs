use std::fmt::Debug;

use anyhow::Result;
use ratchet_rs::{Extension, HeaderMap, UpgradedServer, WebSocket, WebSocketUpgrader};
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
      if let Some(stripped) = i.strip_prefix("I=") {
        cookie_i = stripped.trim().to_string();
      }
    }
  };

  if cookie_i.is_empty() {
    return Ok(None);
  }

  let _uid = b64_u64(uri);

  let mut headers = HeaderMap::new();
  headers.insert("xxx", "abc".parse()?);

  // let response = WebSocketResponse::with_headers(200, headers);
  let UpgradedServer { websocket, .. } = upgrader.upgrade_with(headers).await?;
  // let UpgradedServer { websocket, .. } = upgrader.upgrade().await?;
  Ok(Some(websocket))
}
