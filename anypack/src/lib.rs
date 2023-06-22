#![feature(min_specialization)]

pub use axum::http::HeaderValue;
mod tuple;
use std::ops::Deref;

use axum::response::{IntoResponse, Response};
pub use msgpacker::{pack_array, Packable};
use paste::paste;

macro_rules! any_from {
    ($cls:ident) => {
        impl From<$cls> for Any {
            fn from(v: $cls) -> Self {
                paste! {
                    Self::[< $cls:camel >](v)
                }
            }
        }
    };
    ($($cls:ident),+) => {
        $(
            any_from!($cls);
        )+
    };
}

macro_rules! any {
    ($($cls:ident),+) => {
        paste! {
            #[derive(Clone, Debug)]
            pub enum Any {
                $([<$cls:camel>]($cls),)+
            }
        }

        any_from!($($cls),+);
        impl Packable for Any {
            fn pack<T>(&self, vec: &mut T) -> usize
            where
                T: Extend<u8>,
            {
                paste! {
                    match self {
                        $(Any::[< $cls:camel>](t)=>Packable::pack(&t, vec)),+
                    }
                }
            }
        }
    };
}

impl<T: Into<Any>> From<Vec<T>> for Any {
  default fn from(li: Vec<T>) -> Self {
    let mut r = VecAny::new();
    for i in li {
      r.push(i.into());
    }
    r.into()
  }
}

impl From<&str> for Any {
  fn from(v: &str) -> Self {
    v.to_string().into()
  }
}

impl From<Any> for Response {
  fn from(v: Any) -> Self {
    let mut r = Vec::new();
    v.pack(&mut r);
    response(r)
  }
}

pub type VecU8 = Vec<u8>;

#[derive(Clone, Debug)]
pub struct VecAny(Vec<Any>);

impl Packable for VecAny {
  fn pack<T>(&self, buf: &mut T) -> usize
  where
    T: Extend<u8>,
  {
    pack_array(buf, self.0.clone())
  }
}

any!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, String, VecU8, VecAny);

impl Deref for VecAny {
  type Target = Vec<Any>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub fn response(r: impl IntoResponse) -> Response {
  let mut r = IntoResponse::into_response(r);
  r.headers_mut()
    .insert(axum::http::header::CONTENT_TYPE, MSGPACK.clone());
  r
}

pub trait Pack {
  fn pack(self) -> Vec<u8>;
}

impl<A: IntoIterator<IntoIter = I>, I: Iterator<Item = V> + ExactSizeIterator, V: Packable> Pack
  for A
{
  fn pack(self) -> Vec<u8> {
    let mut buf = Vec::new();
    pack_array(&mut buf, self);
    buf
  }
}

impl VecAny {
  pub fn push(&mut self, val: impl Into<Any>) {
    self.0.push(val.into())
  }
  pub fn new() -> Self {
    Self(Vec::new())
  }
}

pub static MSGPACK: HeaderValue = HeaderValue::from_static("m");

impl From<VecAny> for Response {
  fn from(v: VecAny) -> Self {
    v.0.pack().into_response()
  }
}

impl Default for VecAny {
  fn default() -> Self {
    Self::new()
  }
}

pub fn pack<T: From<Any>>(any: impl Into<Any>) -> awp::Result<T> {
  Ok(any.into().into())
}
