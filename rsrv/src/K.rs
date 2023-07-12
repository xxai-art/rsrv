use xxai::u64_bin;

macro_rules! key {
    ($($key:ident),+) => {
        $(
            paste! {
                pub static [<$key:snake:upper>]: &'static [u8] = stringify!($key).as_bytes();
            }
        )+
    };
}

// key!(favSum, favId);

pub fn nchan(uid: u64) -> Vec<u8> {
  [&b"nchan:"[..], &u64_bin(uid)].concat()
}
