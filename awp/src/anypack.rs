use std::{future::Future, pin::Pin};

pub use anypack::Any;
use axum::{
  body::BoxBody,
  extract::{FromRequest, FromRequestParts},
  handler::Handler,
  http::request::Request,
  response::{IntoResponse, Response},
};

#[macro_export]
macro_rules! any {
  ()=>{
    awp::Result<impl Into<awp::anypack::AnyResult>>
  }
}

#[derive(Clone)]
pub struct FnAny<F>(pub F);

pub type Result<T> = crate::Result<T, crate::Err>;

// pub fn into_response(result: Result<impl Into<Any>>) -> Response {
//   match result {
//     Ok(r) => r.into().into_response(),
//     Err(err) => err.into_response(),
//   }
// }

#[derive(Debug)]
pub enum AnyResult {
  Any(Any),
  Response(Response),
}

impl<T: Into<Any>> From<T> for AnyResult {
  fn from(t: T) -> Self {
    AnyResult::Any(t.into())
  }
}

impl<T: Future<Output = crate::Result<A>>, A: Into<Any>> From<T> for AnyResult {
  fn from(t: T) -> Self {
    todo!()
  }
}

impl IntoResponse for AnyResult {
  fn into_response(self) -> Response<BoxBody> {
    match self {
      AnyResult::Any(r) => r.into_response(),
      AnyResult::Response(r) => r,
    }
  }
}

pub async fn await_into_response(
  result: impl Future<Output = Result<impl Into<AnyResult>>> + Send,
) -> Response {
  match result.await {
    Ok(r) => r.into().into_response(),
    Err(err) => err.into_response(),
  }
}

impl<F, Fut, S, B, T: Into<AnyResult>> Handler<((),), S, B> for FnAny<F>
where
  F: FnOnce() -> Fut + Clone + Send + 'static,
  Fut: Future<Output = Result<T>> + Send,
  B: Send + 'static,
{
  type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

  fn call(self, _req: Request<B>, _state: S) -> Self::Future {
    Box::pin(async move { await_into_response(self.0()).await })
  }
}

macro_rules! impl_handler {
    (
        [$($ty:ident),*], $last:ident
    ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<F, Fut, S, B,  M, T: Into<AnyResult>, $($ty,)* $last> Handler<(M, $($ty,)* $last,), S, B> for FnAny<F>
            where
                F: FnOnce($($ty,)* $last,) -> Fut + Clone + Send + 'static,
                Fut: Future<Output = Result<T>> + Send,
                B: Send + 'static,
                S: Send + Sync + 'static,
                $( $ty: FromRequestParts<S> + Send, )*
                    $last: FromRequest<S, B, M> + Send,
                {
                    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

                    fn call(self, req: Request<B>, state: S) -> Self::Future {
                        Box::pin(async move {
                            let (mut parts, body) = req.into_parts();
                            let state = &state;

                            $(
                                let $ty = match $ty::from_request_parts(&mut parts, state).await {
                                    Ok(value) => value,
                                    Err(rejection) => return rejection.into_response(),
                                };
                            )*

                                let req = Request::from_parts(parts, body);

                            let $last = match $last::from_request(req, state).await {
                                Ok(value) => value,
                                Err(rejection) => return rejection.into_response(),
                            };

                            await_into_response(self.0($($ty,)* $last,)).await

                        })
                    }
                }
    };
}

macro_rules! all_the_tuples {
  ($name:ident) => {
    $name!([], T1);
    $name!([T1], T2);
    $name!([T1, T2], T3);
    $name!([T1, T2, T3], T4);
    $name!([T1, T2, T3, T4], T5);
    $name!([T1, T2, T3, T4, T5], T6);
    $name!([T1, T2, T3, T4, T5, T6], T7);
    $name!([T1, T2, T3, T4, T5, T6, T7], T8);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9], T10);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], T11);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
    $name!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
    $name!(
      [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13],
      T14
    );
    $name!(
      [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14],
      T15
    );
    $name!(
      [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15],
      T16
    );
  };
}

all_the_tuples!(impl_handler);
