use std::fmt::Debug;

use anyhow::Result;
use ratchet_rs::{Extension, HeaderMap, UpgradedServer, WebSocket, WebSocketUpgrader};
use tokio::net::TcpStream;
use xuser::ClientUser;

pub async fn header_user<T: Extension + Debug>(
  upgrader: WebSocketUpgrader<TcpStream, T>,
) -> Result<(String, ClientUser, WebSocket<TcpStream, T>)> {
  let req = upgrader.request();
  let uri = req.uri().to_string();

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

  Ok((uri, client_user, websocket))
}
