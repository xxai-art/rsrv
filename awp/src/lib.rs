#![feature(min_specialization)]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

pub mod anypack;
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
