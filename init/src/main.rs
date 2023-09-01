use std::{env, fs};

use anyhow::Error;
use gt::QE;

fn remove_comment<S: AsRef<str>>(input: S) -> String {
  let lines = input.as_ref().lines();
  let filtered_lines: Vec<String> = lines
    .map(|line| {
      if let Some(pos) = line.find('#') {
        String::from(line[0..pos].trim_end())
      } else {
        String::from(line)
      }
    })
    .filter(|line| !line.is_empty())
    .collect();
  filtered_lines.join("\n")
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  let sql_li = fs::read_to_string("init.sql")?;

  for sql in sql_li.split(';') {
    let sql = sql.trim_end();
    if !sql.is_empty() {
      let sql = sql.to_owned() + ";";
      println!("\n---{}", sql);
      let sql = remove_comment(sql);
      QE(sql, &[]).await?;
    }
  }
  Ok(())
}
