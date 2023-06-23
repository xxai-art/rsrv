#!/usr/bin/env python

print("""use paste::paste;

use crate::{Val, ValLi};

macro_rules! into_val_li {
   ($($t:tt),+)=>{
paste!{
impl<$([<T $t>]: Into<Val>),+> From<($([<T $t>]),+)> for ValLi {
  fn from(t: ($([<T $t>]),+)) -> Self {
    ValLi(vec![$(t.$t.into()),+])
  }
}
}
  }
}
""")

for i in range(2, 128):
  print("into_val_li!(%s);" % ",".join(tuple(map(str, range(i)))))
