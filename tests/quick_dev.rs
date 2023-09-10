#!allow(unused);

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8081")?;

    hc.do_get("/hello").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username" : "dummy_name",
            "pwd": "welcome"
        })
    );

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA"
        }),
    );
    req_create_ticket.await?.print().await?;

    hc.do_delete("/api/tickets/1").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}