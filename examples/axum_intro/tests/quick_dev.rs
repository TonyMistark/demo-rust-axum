// cargo watch -q -c -w tests/ -x "test  -- --nocapture"
use anyhow::Result;

#[tokio::test]
async fn test_hello() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8090")?;

    hc.do_get("/hello").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_hello1() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8090")?;

    hc.do_get("/hello1/?name=ice").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_hello2() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8090")?;

    hc.do_get("/hello2/ice/").await?.print().await?;
    hc.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}
