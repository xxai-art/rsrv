use std::time;

use chrono::{TimeZone, Utc};
pub use sts::{ms, sec};
use tokio::time::interval;

// 获取一个月的第一天的毫秒数
pub fn first_millis_of_month(year: i32, month: u8) -> u64 {
  // 使用 Utc.ymd 创建指定年月的第一天的日期
  let begin_date = Utc.with_ymd_and_hms(year, month as _, 1, 0, 0, 0).unwrap();

  // 使用 timestamp_millis 方法获取日期对应的毫秒级时间戳
  begin_date.timestamp_millis() as u64
}

// 根据年份和月份计算该月起始和结束的时间戳
pub fn ym_ms_range(year: i32, month: u8) -> (u64, u64) {
  // 调用 first_millis_of_month 函数获取该月的第一毫秒
  let begin_ms = first_millis_of_month(year, month);

  // 计算下个月的第一毫秒，然后减去一毫秒，得到当前月的最后一毫秒
  // 注意这里要考虑12月的情况，即如果当前月份是12月，则下个月是下一年的1月
  let end_ms = if month == 12 {
    first_millis_of_month(year + 1, 1)
  } else {
    first_millis_of_month(year, month + 1)
  } - 1;

  // 返回两个时间戳
  (begin_ms, end_ms)
}

pub fn n_to_year_month(n: i32) -> (i32, u8) {
  (n / 12, (n % 12) as _)
}

pub static mut TODAY: u32 = 0;

pub fn today() -> u32 {
  unsafe { TODAY }
}

pub async fn update_today() {
  loop {
    let now = time::SystemTime::now()
      .duration_since(time::UNIX_EPOCH)
      .unwrap()
      .as_secs();

    let today = now / 86400;
    unsafe {
      TODAY = today as u32;
    }

    let next = (1 + today) * 86400 + 1;

    interval(time::Duration::from_secs(next - now)).tick().await;
  }
}
