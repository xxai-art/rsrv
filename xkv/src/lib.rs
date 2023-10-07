use std::{collections::BTreeMap, env, ops::Deref, str::FromStr};

use anyhow::Result;
pub use async_lazy::Lazy;
pub use ctor::ctor;
pub use fred::{
  self,
  interfaces::ClientLike,
  prelude::{ReconnectPolicy, RedisClient, RedisConfig, ServerConfig},
};
pub use lazy_static::lazy_static;
pub use paste::paste;
pub use trt::TRT;
pub struct Server {
  c: ServerConfig,
}

impl Server {
  pub fn cluster(host_port_li: Vec<(String, u16)>) -> Self {
    Self {
      c: ServerConfig::Clustered {
        hosts: host_port_li
          .into_iter()
          .map(|(host, port)| fred::types::Server::new(host, port))
          .collect(),
      },
    }
  }

  pub fn host_port(host: String, port: u16) -> Self {
    Self {
      c: ServerConfig::Centralized {
        server: fred::types::Server::new(host, port),
      },
    }
  }
}

const USER: &str = "USER";
const HOST_PORT: &str = "HOST_PORT";
const PASSWORD: &str = "PASSWORD";
const RESP: &str = "RESP";
const DATABASE: &str = "DATABASE";

pub struct Wrap(pub &'static Lazy<RedisClient>);

impl Deref for Wrap {
  type Target = RedisClient;
  fn deref(&self) -> &Self::Target {
    self.0.get().unwrap()
  }
}

#[macro_export]
macro_rules! conn {
  ($var:ident = $prefix:ident) => {
    $crate::paste! {
        pub static [<__ $var>]: $crate::Lazy<$crate::RedisClient> = $crate::Lazy::const_new(|| {
            Box::pin(async move { $crate::conn(stringify!($prefix)).await.unwrap() })
        });

        $crate::lazy_static! {
            pub static ref $var:$crate::Wrap = $crate::Wrap(&[<__ $var>]);
        }
        #[$crate::ctor]
        fn [<init_ $prefix:lower>]() {
            $crate::TRT.block_on(async move {
                use std::future::IntoFuture;
                [<__ $var>].into_future().await;
            });
        }
    }
  };
}

pub async fn conn(prefix: impl AsRef<str>) -> Result<RedisClient> {
  let prefix = prefix.as_ref().to_owned() + "_";

  let mut map = BTreeMap::new();

  for (key, value) in env::vars() {
    if key.starts_with(&prefix) {
      let key = &key[prefix.len()..];

      if [USER, HOST_PORT, PASSWORD, RESP, DATABASE].contains(&key) {
        map.insert(key.to_owned(), value.trim().to_owned());
      }
    }
  }

  let host_port = map
    .get(HOST_PORT)
    .unwrap()
    .split(' ')
    .map(|i| i.trim())
    .filter(|i| !i.is_empty())
    .map(|i| {
      let hp = i.split(':').collect::<Vec<_>>();
      if hp.len() == 1 {
        (i.to_owned(), 6379u16)
      } else {
        (hp[0].to_owned(), hp[1].parse::<u16>().unwrap())
      }
    })
    .collect::<Vec<_>>();

  let server = if host_port.len() == 1 {
    let (host, port) = &host_port[0];
    Server::host_port(host.to_string(), *port)
  } else {
    Server::cluster(host_port)
  };

  let database = map.get(DATABASE).map(|s| u8::from_str(s).unwrap());
  let resp = map.get(RESP).map(|s| u8::from_str(s).unwrap());

  connect(
    &server,
    map.get(USER).cloned(),
    map.get(PASSWORD).cloned(),
    database,
    resp,
  )
  .await
}

#[test]
fn test_conn() -> Result<()> {
  use fred::interfaces::KeysInterface;
  use tokio_test::block_on;
  block_on(async move {
    let redis = conn("REDIS").await?;
    let key = "test";
    redis.del(key).await?;
    assert_eq!(redis.get::<Option<String>, _>(key).await?, None);
    let val = "值 abc";
    redis.set(key, val, None, None, false).await?;
    assert_eq!(redis.get::<Option<String>, _>(key).await?, Some(val.into()));
    redis.del(key).await?;
    assert_eq!(redis.get::<Option<String>, _>(key).await?, None);
    Ok::<_, anyhow::Error>(())
  })?;
  Ok(())
}

pub async fn connect(
  server: &Server,
  username: Option<String>,
  password: Option<String>,
  database: Option<u8>,
  resp: Option<u8>,
) -> Result<RedisClient> {
  let resp = match resp {
    Some(v) => {
      if v == 2 {
        fred::types::RespVersion::RESP2
      } else {
        fred::types::RespVersion::RESP3
      }
    }
    None => fred::types::RespVersion::RESP3,
  };
  let mut conf = RedisConfig {
    version: resp,
    ..Default::default()
  };
  conf.server = server.c.clone();
  conf.username = username;
  conf.password = password;
  conf.database = database;
  /*
  https://docs.rs/fred/6.2.1/fred/types/enum.ReconnectPolicy.html#method.new_constant
  */
  let policy = ReconnectPolicy::new_constant(6, 1);
  let client = RedisClient::new(conf, None, None, Some(policy));
  client.connect();
  client.wait_for_connect().await?;
  Ok(client)
}
