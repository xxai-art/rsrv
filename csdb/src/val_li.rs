use crate::Val;

pub struct ValLi(pub Vec<Val>);

impl From<()> for ValLi {
  fn from(_: ()) -> Self {
    ValLi(vec![])
  }
}

impl<T1: Into<Val>, T2: Into<Val>, T3: Into<Val>> From<(T1, T2, T3)> for ValLi {
  fn from(t: (T1, T2, T3)) -> Self {
    ValLi(vec![t.0.into(), t.1.into(), t.2.into()])
  }
}
