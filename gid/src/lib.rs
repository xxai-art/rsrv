#![feature(lazy_cell)]

use std::sync::LazyLock;

use dashmap::DashMap;

pub fn nanos() -> u64 {
  coarsetime::Clock::now_since_epoch().as_nanos()
}

#[derive(Debug, Default)]
pub struct Gid {
  pub hset: Box<[u8]>,
  pub cache: DashMap<Box<[u8]>, IdMax>,
}

#[derive(Debug, Default)]
pub struct IdMax {
  pub id: u64,
  pub max: u64,
  pub time: u64,
  pub step: u64,
}

pub static GID: LazyLock<Gid> = LazyLock::new(|| Gid {
  hset: (*b"id").into(),
  cache: DashMap::default(),
});

pub const STEP_MAX: u64 = (u16::MAX as u64) * 32;

#[macro_export]
macro_rules! gid {
  ($key:ident) => {{
    use std::cmp::{self, min};

    use x0::{fred::interfaces::HashesInterface, R};
    use $crate::{nanos, GID, STEP_MAX};

    let key = stringify!($key).as_bytes();

    if let Some(mut i) = GID.cache.get_mut(key) {
      if i.id == i.max {
        let now = nanos();
        let step = i.step;
        if now > i.time {
          // 600_000_000_000 十分钟
          let diff = (now - i.time) as f32;
          let need = ((6e11 / diff) * (i.step as f32)) as u64;
          i.step = cmp::max(min(need, STEP_MAX), 1);
          i.time = now;
        } else if i.step < STEP_MAX {
          i.step *= 2;
        }
        let max: u64 = R.hincrby(GID.hset.as_ref(), key, step as _).await.unwrap();
        i.max = max;
        i.id = max - step;
      }
      i.id += 1;
      i.id
    } else {
      let step = 1;
      let max: u64 = R.hincrby(GID.hset.as_ref(), key, step as _).await.unwrap();
      let id = max + 1 - step;
      let i = $crate::IdMax {
        time: $crate::nanos(),
        step,
        max,
        id,
      };
      GID.cache.insert(key.into(), i);
      id
    }
  }};
}

#[test]
fn test() {
  tokio_test::block_on(async move {
    for _ in 1..=3 {
      println!("gid {}", gid!(client));
      std::thread::sleep(std::time::Duration::from_secs(1));
    }
    // dbg!(gid!(client));
    // dbg!(gid!(client));
    // dbg!(gid!(client));
    // dbg!(gid!(client));
  })
}

// const HSET: &[u8] = b"id";
//
// pub async fn gid(key: impl AsRef<str>) -> Result<u64> {
//     let key = key.as_ref();
//     let step = 1;
//     id = max - step;
//     Ok(id)
// }
// < (redis, hset, duration=6e4)=>
//   new Proxy(
//     {}
//     get:(_, name)=>
//       + cache
//       =>
//         if cache
//           [id,max] = cache
//           if id == max
//             [step,time] = cache.slice(2)
//             now = + new Date
//             diff = now - time
//             if diff > duration
//               if step > 1
//                 --step
//             else
//               step += Math.round(
//                 duration / Math.max(diff,1e3)
//               )
//
//             max = await redis.hincrby(hset, name, step)
//             id = max - step
//             cache = [id,max,step,now]
//         else
//           step = 1
//           max = await redis.hincrby(hset, name, step)
//           id = max - step
//           cache = [id,max,step,+new Date]
//         return ++cache[0]
//   )
