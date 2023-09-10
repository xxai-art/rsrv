use std::sync::Arc;

use dashmap::DashMap;
use ratchet_rs::deflate::DeflateEncoder;
use tokio::{net::TcpStream, sync::Mutex};

pub type Sender = Arc<Mutex<ratchet_rs::Sender<TcpStream, DeflateEncoder>>>;

pub type UserWs = DashMap<u64, Sender>;

#[derive(Clone, Default)]
pub struct AllWs(Arc<DashMap<u64, UserWs>>);

impl AllWs {
  pub fn to_user() {}
  pub fn to_user_client_id() {}

  pub fn insert(&self, uid: u64, client_id: u64, sender: Sender) {
    self
      .0
      .entry(uid)
      .or_default()
      .insert(client_id, sender.clone());
  }

  pub fn remove(&self, uid: u64, client_id: u64) {
    let d = &self.0;
    if let Some(map) = d.get(&uid) {
      if map.len() > 1 {
        map.remove(&client_id);
        return;
      }
    };

    d.remove(&uid);
  }
}
