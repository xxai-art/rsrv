#![feature(lazy_cell)]

use std::{
  cell::LazyCell,
  collections::HashMap,
  sync::{Arc, LazyLock},
};

use anyhow::Result;
use dashmap::DashMap;
use x0::{fred::interfaces::HashesInterface, R};

#[derive(Debug, Default)]
pub struct IdMax {
  pub id: u64,
  pub max: u64,
  pub time: u64,
}

#[derive(Debug, Default)]
pub struct Gid {
  pub hset: Box<[u8]>,
  pub cache: DashMap<Box<[u8]>, IdMax>,
}

pub static GID: LazyLock<Gid> = LazyLock::new(|| Gid {
  hset: (*b"id").into(),
  cache: DashMap::default(),
});

#[macro_export]
macro_rules! gid {
  ($key:ident) => {{
    let key = stringify!($key).as_bytes();
    use $crate::GID;
    if let Some(mut i) = GID.cache.get_mut(key) {
      if i.id < i.max {
        i.id += 1;
        i.id
      } else {
        0
      }
    } else {
      0
    }
    // if GID.id == 0 {
    //   GID.lock().id = 1;
    // }
    // GID.id
  }};
}

// const HSET: &[u8] = b"id";
//
// pub async fn gid(key: impl AsRef<str>) -> Result<u64> {
//     let key = key.as_ref();
//     let step = 1;
//     let max: u64 = R.hincrby(HSET, key, step).await?;
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
