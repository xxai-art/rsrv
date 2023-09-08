pub mod action;
pub mod cid;
mod ws;
pub use ws::{WR, WS};

pub const CLIP: &str = "clip";

pub mod 分级 {
  pub const 不限: u8 = 2;
  pub const 安全: u8 = 1;
  pub const 成人: u8 = 0;
}
