use crate::Val;

pub struct ValLi(pub Vec<Val>);

impl From<()> for ValLi {
  fn from(_: ()) -> Self {
    ValLi(vec![])
  }
}

impl<T: Into<Val>> From<T> for ValLi {
  fn from(t: T) -> Self {
    ValLi(vec![t.into()])
  }
}
