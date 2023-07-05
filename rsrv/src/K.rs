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

key!(favSum, favId);
