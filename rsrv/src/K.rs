use intbin::u64_bin;
use paste::paste;

macro_rules! key {
    ($($key:ident),+) => {
        $(
            paste! {
                pub static [<$key:snake:upper>]: &'static [u8] = stringify!($key).as_bytes();
            }
        )+
    };
}

key!(favLast, seenLast, rec, rec0, rec1);

pub fn nchan(uid: u64) -> Vec<u8> {
  [&b"nchan:"[..], &u64_bin(uid)].concat()
}
