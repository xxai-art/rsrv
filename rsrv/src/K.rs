use intbin::u64_bin;
use paste::paste;

macro_rules! key {
    ($($key:ident),+) => {
        $(
            paste! {
                pub const [<$key:snake:upper>]: &'static [u8] = stringify!($key).as_bytes();
            }
        )+
    };
}

key!(favLast, seenLast, r, r0, r1, hr2);

pub const REC: &[&[u8]; 3] = &[R0, R1, R];

pub fn nchan(uid: u64) -> Vec<u8> {
  [&b"nchan:"[..], &u64_bin(uid)].concat()
}
