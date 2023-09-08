use int_enum::IntEnum;

pub enum WS {
  未登录 = 0,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum WR {
  同步 = 0,
}
