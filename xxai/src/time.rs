use chrono::{Datelike, TimeZone, Utc};

// 根据年份和月份计算该月起始和结束的时间戳
fn ym_ms_range(year: i32, month: u32) -> (u64, u64) {
  // 使用 Utc.ymd 创建指定年月的第一天的日期
  let begin_date = Utc.ymd(year, month, 1);

  // 使用 with_month 计算下个月的日期
  // 注意 with_month 返回一个 Option，所以需要使用 unwrap_or_else 处理错误情况
  let next_month_date = begin_date
    .with_month(month % 12 + 1)
    .unwrap_or_else(|| Utc.ymd(year + 1, 1, 1));

  // 由于 Rust 的时间戳默认单位为秒，所以我们需要把这两个时间转换为毫秒级的时间戳
  // 使用 timestamp_millis 方法获取日期对应的毫秒级时间戳
  let begin_ms = begin_date.and_hms(0, 0, 0).timestamp_millis() as u64;

  // 计算当前月的最后一毫秒的时间戳
  // 需要先减去一毫秒，得到最后一毫秒
  let end_ms = next_month_date.and_hms(0, 0, 0).timestamp_millis() as u64 - 1;

  // 返回两个时间戳
  (begin_ms, end_ms)
}

fn main() {
  // 测试函数
  let (begin_ms, end_ms) = ym_ms_range(2023, 7);
  println!("For 2023, 7, the first millisecond is: {}", begin_ms);
  println!("For 2023, 7, the last millisecond is: {}", end_ms);
}
