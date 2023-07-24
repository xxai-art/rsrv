use tokio_postgres::{error::SqlState, Client, NoTls};

pub async fn conn(env: impl Into<String>) -> Client {
  let env = env.into();
  let pg_uri = std::env::var(&env).unwrap();
  let (client, connection) = tokio_postgres::connect(&format!("postgres://{}", pg_uri), NoTls)
    .await
    .unwrap();

  tokio::spawn(async move {
    if let Err(e) = connection.await {
      let err_code = e.code();
      let code = match err_code {
        Some(code) => code.code(),
        None => "",
      };
      tracing::error!("❌ {env} ERROR CODE {code} : {e}");

      if err_code == Some(&SqlState::ADMIN_SHUTDOWN) || e.is_closed() {
        std::process::exit(1)
      }
    }
  });

  client
}
