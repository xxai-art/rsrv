use anyhow::Result;
use x0::{fred::interfaces::HashesInterface, R};

const HSET: &[u8] = b"id";

pub async fn gid(key: impl AsRef<str>) -> Result<u64> {
  let key = key.as_ref();
  let step = 1;
  let max: u64 = R.hincrby(HSET, key, step).await?;
  Ok(max)
}
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
//
//
