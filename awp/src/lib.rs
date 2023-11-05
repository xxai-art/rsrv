#![feature(min_specialization)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use axum::{http::StatusCode, response::IntoResponse, Extension};

pub mod anypack;
mod err;
mod log;
mod srv;

pub use err::{Err, Error, Result};
pub use log::init;
pub use srv::srv;

pub type Response = Result<axum::response::Response>;

pub type E<T> = Extension<T>;

pub fn ok() -> Response {
  let r = (StatusCode::OK, "").into_response();
  Ok(r)
}
