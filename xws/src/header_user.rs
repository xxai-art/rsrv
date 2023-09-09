use std::fmt::Debug;

use anyhow::Result;
use ratchet_rs::{Extension, HeaderMap, UpgradedServer, WebSocket, WebSocketUpgrader};
use tokio::net::TcpStream;
use ub64::b64_u64;
use xuser::ClientUser;

pub async fn header_user<T: Extension + Debug>(
  upgrader: WebSocketUpgrader<TcpStream, T>,
) -> Result<(ClientUser, WebSocket<TcpStream, T>)> {
  let req = upgrader.request();
  let mut uri = req.uri().to_string();
  if let Some(p) = uri.rfind('/') {
    uri = uri[p + 1..].to_string()
  };

  let headers = req.headers();
  let cookie = if let Some(cookie) = headers.get("cookie") {
    Some(cookie.to_str()?)
  } else {
    None
  };

  let host = headers.get("host").unwrap().to_str()?;
  let (client_user, cookie) = xuser::client_user_cookie(host, cookie).await;

  let UpgradedServer { websocket, .. } = if let Some(cookie) = cookie {
    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", cookie.parse()?);
    upgrader.upgrade_with(headers).await?
  } else {
    upgrader.upgrade().await?
  };

  Ok((client_user, websocket))
}
