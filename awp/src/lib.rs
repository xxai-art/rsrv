#![feature(min_specialization)]
#![feature(impl_trait_in_assoc_type)]

mod env;
mod err;
mod log;
mod srv;

use axum::Extension;
pub use env::env_default;
pub use err::{Err, Error, Result};
pub use log::init;
pub use srv::srv;

pub type Response = Result<axum::response::Response>;

pub type E<T> = Extension<T>;
