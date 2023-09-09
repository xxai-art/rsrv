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

  let headers = req.headers();
  let cookie = if let Some(cookie) = headers.get("cookie") {
    Some(cookie.to_str())
  } else {
    None
  };

  let (client_user, cookie) = xuser::client_user_cookie(headers.get("host").unwrap(), cookie).await;

  let mut headers = HeaderMap::new();
  headers.insert("xxx", "abc".parse()?);

  // let response = WebSocketResponse::with_headers(200, headers);
  let UpgradedServer { websocket, .. } = upgrader.upgrade_with(headers).await?;
  // let UpgradedServer { websocket, .. } = upgrader.upgrade().await?;
  Ok(Some(websocket))
}
