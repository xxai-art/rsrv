use std::net::SocketAddr;

use axum::{
  extract::{
    ws::{Message, WebSocket},
    WebSocketUpgrade,
  },
  response::IntoResponse,
  routing::get,
  Router, TypedHeader,
};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

async fn ws_handler(
  ws: WebSocketUpgrade,
  user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
  if let Some(TypedHeader(user_agent)) = user_agent {
    println!("`{}` connected", user_agent.as_str());
  }

  ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
  if let Some(msg) = socket.recv().await {
    if let Ok(msg) = msg {
      println!("Client says: {:?}", msg);
      //客户端发什么，服务端就回什么（只是演示而已）
      // if socket
      //   .send(Message::Text(format!("{:?}", msg)))
      //   .await
      //   .is_err()
      // {
      //   println!("client disconnected");
      //   return;
      // }
    } else {
      println!("client disconnected");
      return;
    }
  }
}

#[tokio::main]
async fn main() {
  loginit::init();
  let app = Router::new().route("/-:uid", get(ws_handler)).layer(
    TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)),
  );

  let addr = SocketAddr::from(([0, 0, 0, 0], envport::get("PORT", 8132)));

  tracing::info!("ws://{}", addr);

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
