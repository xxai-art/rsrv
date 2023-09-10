use std::sync::Arc;

use dashmap::DashMap;
use ratchet_rs::{deflate::DeflateEncoder, Sender};
use tokio::{net::TcpStream, sync::Mutex};

pub type UserWs = DashMap<u64, Arc<Mutex<Sender<TcpStream, DeflateEncoder>>>>;

pub type AllWs = Arc<DashMap<u64, UserWs>>;
