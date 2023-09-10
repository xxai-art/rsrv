#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;

use volo_grpc::server::{Server, ServiceBuilder};
use xwsrpc::S;

#[volo::main]
async fn main() {
  loginit::init();

  let addr: SocketAddr = match std::env::var("XWSRPC_PORT") {
    Ok(uri) => uri,
    Err(_) => "0.0.0.0:6201".into(),
  }
  .parse()
  .unwrap();

  tracing::info!("grpc://{addr}");
  let addr = volo::net::Address::from(addr);

  Server::new()
    .add_service(ServiceBuilder::new(volo_gen::rpc::RpcServer::new(S)).build())
    .run(addr)
    .await
    .unwrap();
}
