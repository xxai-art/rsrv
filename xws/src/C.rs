use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum SEND {
  收藏 = 0,
  浏览 = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum RECV {
  同步 = 0,
}
