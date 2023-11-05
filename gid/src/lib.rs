pub use anyhow;
pub use ctor;
pub use fred;
pub use paste::paste;
pub use tokio;
pub use trt;
pub use x0::R;

pub fn nanos() -> u64 {
  coarsetime::Clock::now_since_epoch().as_nanos()
}

// #[derive(Debug, Default)]
// pub struct Gid {
//   pub hset: Box<[u8]>,
//   pub cache: DashMap<Box<[u8]>, IdMax>,
// }
//
#[derive(Debug, Default)]
pub struct IdMax {
  pub id: u64,
  pub max: u64,
  pub time: u64,
  pub step: u64,
}
//
// pub static GID: LazyLock<Gid> = LazyLock::new(|| Gid {
//   hset: (*b"gid").into(),
//   cache: DashMap::default(),
// });

pub const STEP_MAX: u64 = u16::MAX as u64;

pub const GID: &'static [u8] = b"gid";

#[macro_export]
macro_rules! gid {
  ($key:ident) => {
    pub mod $key {

      use std::sync::Arc;

      use $crate::{anyhow::Result, tokio::sync::Mutex, IdMax, R};

      pub static ID: Mutex<IdMax> = Mutex::const_new(IdMax {
        id: 0,
        max: 0,
        time: 0,
        step: 1,
      });

      #[macro_export]
      macro_rules! next {
        ($id:ident) => {{
          use std::cmp::min;

          use $crate::{fred::interfaces::HashesInterface, nanos};
          let now = nanos();
          if $id.time > 0 {
            let diff = (now - $id.time) as f32;
            if 6e11 > diff {
              $id.step = min($id.step * 2, $crate::STEP_MAX);
            } else {
              if $id.step > 2 {
                $id.step /= 2
              }
            }
          }

          let step = $id.step;
          let max = R
            .hincrby::<u64, _, _>($crate::GID, stringify!($key), step as _)
            .await?;

          $id.max = max;
          $id.id = max - step;
          $id.time = now;
        }};
      }

      pub fn init() -> Result<()> {
        $crate::trt::TRT.block_on(async move {
          let mut id = ID.lock().await;
          R.0.force().await;
          next!(id);
          Ok(())
        })
      }
    }

    #[$crate::ctor::ctor]
    fn init_gid() {
      $key::init().unwrap();
    }

    $crate::paste! {
        pub async fn [<gid_ $key>]()->$crate::anyhow::Result<u64>{
          let mut id =  $key::ID.lock().await;
          id.id+=1;
          let r = id.id;
          if id.id == id.max {
            next!(id);
          }
          Ok(r)
        }
    }
  };
}
