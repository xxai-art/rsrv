[‼️]: ✏️README.mdt

# csdb

wrap for ceresdb-client-rs

use example :

```rust
use csdb::{conn_by_env, Db, SQL};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DB: Db = conn_by_env("CERESDB_GRPC").unwrap();
    pub static ref SQL_DROP_TEST: SQL = DB.sql(["test"], "DROP TABLE test");

    // ctime 是用户记录创建时间
    // ts 是写入时间
    pub static ref SQL_TEST: SQL = DB.sql(["test"], r#"CREATE TABLE test (
  ts TIMESTAMP NOT NULL,
  uid uint64 NOT NULL,
  tag string NOT NULL,
  TIMESTAMP KEY(ts),
  PRIMARY KEY(uid, ts)
) ENGINE=Analytic WITH (
  compression='ZSTD',
  enable_ttl='false'
)"#);
    pub static ref SQL_INSERT: SQL = DB.sql(["test"], "INSERT INTO test (ts,uid,tag) VALUES ({},{},{})");
    pub static ref SQL_SELECT: SQL = DB.sql(["test"], "SELECT * FROM test");
    // pub static ref SQL_DELETE: SQL = DB.sql(["test"], "DELETE FROM test WHERE ts={} AND uid={}");
}

#[tokio::main]
#[test]
async fn main() -> anyhow::Result<()> {
  loginit::init();

  let _ = SQL_DROP_TEST.exe(()).await;
  SQL_TEST.exe(()).await?;
  SQL_INSERT.exe((1, 2, "test")).await?;
  SQL_INSERT.exe((2, 2, "\'\"\r\n")).await?;

  let li = SQL_SELECT.li(()).await?;
  assert_eq!(li.len(), 2);
  for i in li {
    dbg!(i);
  }
  // SQL_DELETE.exe([1, 3]).await?;
  Ok(())
}
```

output:

```
warning: unused imports: `VarError`, `sync::Arc`, `var`
 --> csdb/src/val.rs:2:9
  |
2 |   env::{var, VarError},
  |         ^^^  ^^^^^^^^
3 |   fmt::{Display, Formatter},
4 |   sync::Arc,
  |   ^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `Builder`, `DbClient`, `Error`, `Mode`, `RpcConfig`, `RpcContext`, `SqlQueryRequest`, `SqlQueryResponse`, `model::sql_query::row::Row`
  --> csdb/src/val.rs:9:3
   |
9  |   model::sql_query::row::Row, Builder, DbClient, Error, Mode, RpcConfig, RpcConte...
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^  ^^^^^  ^^^^  ^^^^^^^^^  ^^^^^^^^^^
10 |   SqlQueryRequest, SqlQueryResponse,
   |   ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^

warning: unused import: `coarsetime::Instant`
  --> csdb/src/val.rs:12:5
   |
12 | use coarsetime::Instant;
   |     ^^^^^^^^^^^^^^^^^^^

warning: unused import: `dyn_fmt::AsStrFormatExt`
  --> csdb/src/val.rs:13:5
   |
13 | use dyn_fmt::AsStrFormatExt;
   |     ^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `error`, `info`
  --> csdb/src/val.rs:15:15
   |
15 | use tracing::{error, info};
   |               ^^^^^  ^^^^

warning: unused import: `paste::paste`
 --> csdb/src/val_li.rs:1:5
  |
1 | use paste::paste;
  |     ^^^^^^^^^^^^

warning: unused imports: `Display`, `Formatter`
 --> csdb/src/lib.rs:6:9
  |
6 |   fmt::{Display, Formatter},
  |         ^^^^^^^  ^^^^^^^^^

warning: unused import: `aho_corasick::AhoCorasick`
  --> csdb/src/lib.rs:10:5
   |
10 | use aho_corasick::AhoCorasick;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `lazy_static::lazy_static`
  --> csdb/src/lib.rs:18:5
   |
18 | use lazy_static::lazy_static;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^

warning: `csdb` (lib) generated 9 warnings (run `cargo fix --lib -p csdb` to apply 9 suggestions)
   Compiling csdb v0.1.2 (/Users/z/wac.tax/rsrv/csdb)
    Finished test [unoptimized + debuginfo] target(s) in 0.69s
     Running tests/test.rs (/Users/z/wac.tax/rsrv/target/debug/deps/test-4bcd22aab6c7ca80)

running 1 test
  INFO csdb: 9ms DROP TABLE test
  INFO csdb: 3ms CREATE TABLE test (
  ts TIMESTAMP NOT NULL,
  uid uint64 NOT NULL,
  tag string NOT NULL,
  TIMESTAMP KEY(ts),
  PRIMARY KEY(uid, ts)
) ENGINE=Analytic WITH (
  compression='ZSTD',
  enable_ttl='false'
)
  INFO csdb: 4ms INSERT INTO test (ts,uid,tag) VALUES (1,2,'test')
  INFO csdb: 4ms INSERT INTO test (ts,uid,tag) VALUES (2,2,'\'"\r\n')
  INFO csdb: 5ms SELECT * FROM test
[csdb/tests/test.rs:38] i = Row {
    columns: [
        Column {
            name: "uid",
            value: UInt64(
                2,
            ),
        },
        Column {
            name: "ts",
            value: Timestamp(
                1,
            ),
        },
        Column {
            name: "tag",
            value: String(
                "test",
            ),
        },
    ],
}
[csdb/tests/test.rs:38] i = Row {
    columns: [
        Column {
            name: "uid",
            value: UInt64(
                2,
            ),
        },
        Column {
            name: "ts",
            value: Timestamp(
                2,
            ),
        },
        Column {
            name: "tag",
            value: String(
                "'\"\r\n",
            ),
        },
    ],
}
test main ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

in this output

```
INFO csdb: 4ms INSERT INTO test (ts,uid,tag) VALUES (1,2,'test')
```

means cost 4ms for this query
