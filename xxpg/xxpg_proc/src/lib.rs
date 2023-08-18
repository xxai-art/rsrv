use std::cmp;

use lazy_static::lazy_static;
use pgw::Pg;
use proc_macro::TokenStream;
use regex::Regex;
use trt::TRT;

lazy_static! {
  static ref RE: Regex = Regex::new(r"\$(\d+)").unwrap();
  static ref PG: Pg = Pg::new_with_env("PG_URI");
}

fn max_n(s: &str) -> usize {
  let mut max = 0;

  for cap in RE.captures_iter(s) {
    let num: usize = cap[1].parse().unwrap();
    max = cmp::max(max, num);
  }

  max
}

fn _q(q: &str, input: TokenStream) -> TokenStream {
  let mut macro_rules = String::new();
  let mut f = String::new();

  for s in input.to_string().split(';') {
    if let Some(pos) = s.find(':') {
      let var = &s[..pos].trim();
      let mut sql = s[pos + 1..]
        .trim()
        .replace(", ", ",")
        .replace(" :: ", "::")
        .replace(" = ", "=")
        .replace(" / ", "/")
        .replace(" > ", ">")
        .replace(" < ", "<")
        .replace("\r\n", " ")
        .replace(['\n', '\r'], " ");

      let is_str = sql.starts_with('"');
      if is_str {
        sql = sql[1..sql.len() - 1].to_string()
      }

      let escaped_sql = if is_str {
        sql.clone()
      } else {
        sql.replace('\"', "\\\"")
      };

      let up = var.to_uppercase();

      macro_rules.push_str(&format!(
        "pub static ref SQL_{up}: xxpg::Sql = xxpg::PG.sql(\"{escaped_sql}\");"
      ));

      let mut result = String::new();
      let mut row_get = String::new();
      let ref_sql = &sql;
      let prepare = TRT.block_on(async move { PG.prepare(ref_sql).await.unwrap() });

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
        row_get.push_str(&format!("r.get::<_,{t}>({pos})"));
        result += &t;
      }

      let mut arg_li = String::new();
      let mut array = String::new();
      let mut type_li = String::new();

      let n = max_n(&sql);
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
          arg_li += &format!("{v}:{t}");
          array.push('&');
          array += v;
        }
        type_li = format!("<{type_li}>");
      }

      let mut body = format!("xxpg::{q}(&*SQL_{up}, &[{array}]).await");
      if result.is_empty() {
        body = format!("{body}?;\n  Ok(())");
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
        } else if q == "Q01" {
          result = format!("Option<{result}>");
          body =
            format!("Ok(match {body}?{{\n    Some(r)=>Some({row_get}),\n    None=>None\n  }})");
        }
      }

      let fn_var = format!(
        "\npub async fn {var}{type_li}({arg_li}) -> std::result::Result<{result}, xxpg::Error>"
      );
      let func = &format!("{fn_var} {{\n  {body}\n}}");
      // println!("\n❯ {var} → {result} :\n{sql}");
      f += func;
    }
  }
  let s = if !f.is_empty() {
    format!("xxpg::lazy_static!{{\n{macro_rules}\n}}\n{f}")
  } else {
    "".to_string()
  };
  // println!("\n\n{s}\n\n");
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

#[allow(non_snake_case)]
#[proc_macro]
pub fn Q01(input: TokenStream) -> TokenStream {
  _q("Q01", input)
}
