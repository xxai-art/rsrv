use xxpg::Q;

Q! {
    li:
        SELECT cid,rid,ts FROM fav.user ORDER BY ts DESC LIMIT 10
}

#[tokio::test]
async fn main() -> anyhow::Result<()> {
  let r = li().await?;
  dbg!(r);
  Ok(())
}
