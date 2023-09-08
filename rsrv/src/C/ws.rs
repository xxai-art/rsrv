use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum WS {
  未登录 = 0,
  同步 = 1,
  浏览 = 2,
  收藏 = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum WR {
  同步 = 0,
}
