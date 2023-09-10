use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum SEND {
  同步完成 = 0,
  收藏 = 1,
  浏览 = 2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum RECV {
  同步 = 0,
}
