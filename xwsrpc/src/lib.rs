use volo_gen::rpc::{DayRange, Level, Point, QIn, QOut, Rpc};
use volo_macro::volo;

async fn clip(msg: QIn) -> anyhow::Result<QOut> {
  Ok(QOut { li: vec![] })
}

volo!(
    Rpc
    clip(self, req:QIn) -> QOut { clip( req.into_inner()).await? }
);
