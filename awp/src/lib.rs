#![feature(min_specialization)]
#![feature(impl_trait_in_assoc_type)]

mod env;
mod err;
mod log;
mod srv;

pub use env::env_default;
pub use err::{Err, Error, Result};
pub use log::init;
pub use srv::srv;

pub type Response = Result<axum::response::Response>;

use axum::Extension;
pub type E<T> = Extension<T>;

use std::convert::Infallible;

use axum::{
  body::HttpBody,
  handler::Handler,
  routing::{post, MethodRouter},
};

pub fn pack<H, T, S, B>(handler: H) -> MethodRouter<S, B, Infallible>
where
  H: Handler<T, S, B>,
  B: HttpBody + Send + 'static,
  T: 'static,
  S: Clone + Send + Sync + 'static,
{
  post(handler)
}
