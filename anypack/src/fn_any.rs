use std::{future::Future, pin::Pin};

use axum::{
  extract::{FromRequest, FromRequestParts},
  handler::Handler,
  http::request::Request,
  response::{IntoResponse, Response},
};

use crate::Any;

#[derive(Clone)]
pub struct FnAny<F>(pub F);

impl<F, Fut, Res, S, B> Handler<((),), S, B> for FnAny<F>
where
  F: FnOnce() -> Fut + Clone + Send + 'static,
  Fut: Future<Output = Res> + Send,
  Res: Into<Any>,
  B: Send + 'static,
{
  type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

  fn call(self, _req: Request<B>, _state: S) -> Self::Future {
    Box::pin(async move { self.0().await.into().into_response() })
  }
}

macro_rules! impl_handler {
  (
    [$($ty:ident),*], $last:ident
  ) => {
    #[allow(non_snake_case, unused_mut)]
    impl<F, Fut, S, B, Res, M, $($ty,)* $last> Handler<(M, $($ty,)* $last,), S, B> for FnAny<F>
      where
        F: FnOnce($($ty,)* $last,) -> Fut + Clone + Send + 'static,
        Fut: Future<Output = Res> + Send,
        B: Send + 'static,
        S: Send + Sync + 'static,
        Res: Into<Any>,
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

              let res = self.0($($ty,)* $last,).await;

              res.into().into_response()
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
