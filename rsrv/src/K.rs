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

// seenLast,

key!(favLast, r, r0, r1, hr2);

pub const REC: &[&[u8]; 3] = &[R0, R1, R];
