// cargo watch -q -c -w tests/ -x "test  -- --nocapture"
use anyhow::Result;
use serde_json::json;

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

#[tokio::test]
async fn test_login_fail() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8090")?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "456",
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_login_success() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8090")?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "123",
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn test_ticket_cases() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8090")?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "123",
        }),
    );
    req_login.await?.print().await?;

    // test_create
    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA",
        }),
    );
    req_create_ticket.await?.print().await?;

    // test get
    hc.do_get("/api/tickets").await?.print().await?;

    // test delete
    hc.do_delete("/api/tickets/1").await?.print().await?;

    Ok(())
}
