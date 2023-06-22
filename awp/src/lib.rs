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
