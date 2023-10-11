use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:9100")?;
    client.do_get("/api/hello").await?.print().await?;
    client.do_get("/api/orders").await?.print().await?;
    client
        .do_get("/api/order/items?order_id=1")
        .await?
        .print()
        .await?;
    Ok(())
}
