use std::sync::Arc;

use anyhow::Result;
use dashmap::DashMap;
use ratchet_rs::{deflate::DeflateEncoder, PayloadType};
use tokio::{net::TcpStream, sync::Mutex};

use crate::C::SEND;

pub type Sender = Arc<Mutex<ratchet_rs::Sender<TcpStream, DeflateEncoder>>>;

pub type UserWs = DashMap<u64, Sender>;

#[derive(Clone, Default)]
pub struct AllWs(Arc<DashMap<u64, UserWs>>);

pub async fn send(sender: &Sender, msg: &[u8]) -> Result<()> {
  sender.lock().await.write(msg, PayloadType::Binary).await?;
  Ok(())
}

fn msg_payload(kind: SEND, payload: &[u8]) -> Vec<u8> {
  let kind = &[kind as u8][..];
  [kind, payload].concat()
}

impl AllWs {
  pub async fn to_user(&self, uid: u64, kind: SEND, payload: &[u8]) -> Result<()> {
    let msg = msg_payload(kind, payload);

    if let Some(map) = self.0.get(&uid) {
      for sender in map.iter() {
        send(&sender, &msg).await?;
      }
    }
    Ok(())
  }

  pub async fn to_client(
    &self,
    uid: u64,
    client_id: u64,
    kind: SEND,
    payload: &[u8],
  ) -> Result<()> {
    let msg = msg_payload(kind, payload);
    if let Some(map) = self.0.get(&uid) {
      if let Some(sender) = map.get(&client_id) {
        return send(&sender, &msg).await;
      }
    }
    Ok(())
  }

  pub fn insert(&self, uid: u64, client_id: u64, sender: Sender) {
    self
      .0
      .entry(uid)
      .or_default()
      .insert(client_id, sender.clone());
  }

  pub fn remove(&self, uid: u64, client_id: u64) {
    let d = &self.0;
    let is_empty;
    if let Some(map) = d.get(&uid) {
      map.remove(&client_id);
      is_empty = map.is_empty();
    } else {
      return;
    }
    // 不能在get中remove，不然会死锁
    if is_empty {
      d.remove_if(&uid, |_, v| v.is_empty());
    }
  }
}
