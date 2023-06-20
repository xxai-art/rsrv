use std::cmp;

use async_lazy::Lazy;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use regex::Regex;
use tokio_postgres::{Client, NoTls};
use trt::TRT;

lazy_static! {
  static ref RE: Regex = Regex::new(r"\$(\d+)").unwrap();
}

static PG: Lazy<Client> = Lazy::const_new(|| {
  let pg_uri = std::env::var("PG_URI").unwrap();
  Box::pin(async move {
    let (client, connection) = tokio_postgres::connect(&format!("postgres://{}", pg_uri), NoTls)
      .await
      .unwrap();
    tokio::spawn(async move {
      if let Err(e) = connection.await {
        eprintln!("postgres connection error: {e}");
      }
    });

    client
  })
});

fn max_n(s: &str) -> usize {
  let mut max = 0;

  for cap in RE.captures_iter(s) {
    let num: usize = cap[1].parse().unwrap();
    max = cmp::max(max, num);
  }

  max
}

fn _q(q: &str, input: TokenStream) -> TokenStream {
  let mut r = String::new();
  let mut f = String::new();

  for s in input.to_string().split(';') {
    if let Some(pos) = s.find(':') {
      let var = &s[..pos].trim();
      let sql = &s[pos + 1..]
        .trim()
        .replace(" :: ", "::")
        .replace(", ", ",")
        .replace("\r\n", " ")
        .replace(['\n', '\r'], " ");
      if !r.is_empty() {
        r.push(',');
      }

      r.push_str(&format!("sql_{var}:\"{sql}\""));

      println!("\n❯ {var} :\n{sql}");

      let mut result = String::new();
      let mut row_get = String::new();
      let prepare = TRT.block_on(async move { PG.force().await.prepare(sql).await.unwrap() });

      let columns = prepare.columns();
      let columns_len = columns.len();

      for (pos, i) in columns.iter().enumerate() {
        let mut col_type = i.type_().name();
        if !result.is_empty() {
          result.push(',');
        }
        let vec = col_type.starts_with('_');
        let mut t = String::new();
        if vec {
          col_type = &col_type[1..];
          t.push_str("Vec<");
        }
        t.push_str(match col_type {
          "bytea" => "Vec<u8>",
          "int8" => "i64",
          "int4" => "i32",
          "int2" => "i16",
          "float4" => "f32",
          "float8" => "f64",
          "text" => "String",
          _ => col_type,
        });
        if vec {
          t.push('>')
        }
        if pos > 0 {
          row_get.push_str(",\n");
        }
        row_get.push_str(&format!("  r.get::<_,{t}>({pos})"));
        result += &t;
      }

      let mut arg_li = String::new();
      let mut array = String::new();
      let mut type_li = String::new();

      let n = max_n(sql);
      if n > 0 {
        let mut i = 0;

        while i < n {
          if i > 0 {
            arg_li.push(',');
            type_li.push(',');
            array.push(',');
          }
          i += 1;
          let t = &format!("T{i}");
          let v = &format!("_{i}");
          type_li += &format!("{t}:xxpg::ToSql+Sync");
          arg_li += &format!("{v}:&{t}");
          array += v;
        }
        type_li = format!("<{type_li}>");
      }

      let up = var.to_uppercase();
      let mut body = format!("xxpg::{q}(SQL_{up}, &[{array}]).await");
      if result.is_empty() {
        body.push(';');
        result = "()".into()
      } else {
        if columns_len > 1 {
          result = format!("({result})");
          row_get = format!("(\n{row_get}\n  )");
        }
        if q == "Q" {
          result = format!("Vec<{result}>");
          body = format!("Ok({body}?.iter().map(|r|{row_get}).collect())");
        } else if q == "Q1" {
          body = format!("let r = {body}?;\n  Ok({row_get})");
        }
      }

      let fn_var =
        format!("\npub async fn {var}{type_li}({arg_li}) -> Result<{result}, xxpg::Error>");
      let func = &format!("{fn_var} {{\n  {body}\n}}");
      println!("{fn_var}\n");
      f += func;
    }
  }
  let s = if !f.is_empty() {
    format!("xxpg::sql!({r});\n{f}")
  } else {
    "".to_string()
  };
  s.parse::<proc_macro2::TokenStream>().unwrap().into()
}

#[allow(non_snake_case)]
#[proc_macro]
pub fn Q(input: TokenStream) -> TokenStream {
  _q("Q", input)
}

#[allow(non_snake_case)]
#[proc_macro]
pub fn Q1(input: TokenStream) -> TokenStream {
  _q("Q1", input)
}
