use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum SEND {
  浏览器同步服务器完成 = 0,
  收藏 = 1,
  浏览 = 2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum RECV {
  浏览器同步服务器 = 0,
  服务器同步浏览器 = 1,
}
