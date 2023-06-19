use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use lazy_static::lazy_static;
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
  pub cache: Arc<HashMap<Box<[u8]>, IdMax>>,
}

lazy_static! {
  pub static ref GID: Gid = Gid {
    hset: (*b"id").into(),
    cache: Arc::default(),
  };
}

#[macro_export]
macro_rules! gid {
  ($key:ident) => {{
    let key = stringify!($key).as_bytes();
    use $crate::GID;
    if let Some(i) = GID.cache.get(key) {
      if i.id < i.max {
        //   i.id += 1;
        //   return i.id;
      }
    }
    // if GID.id == 0 {
    //   GID.lock().id = 1;
    // }
    // GID.id
    1
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
