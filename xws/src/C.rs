use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum SEND {
  服务器传浏览器完成 = 0,
  收藏 = 1,
  浏览 = 2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum RECV {
  服务器传浏览器 = 0,
  浏览器传服务器 = 1,
  用户行为日志 = 2,
}
