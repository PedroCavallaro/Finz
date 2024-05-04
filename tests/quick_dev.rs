use std::{env, os::unix::net::SocketAddr};

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(9999);

    let hc = httpc_test::new_client(&SocketAddr::from(([0, 0, 0, 0], port)))?;

    hc.do_get("/health-check").await?.print().await?;

    Ok(())
}
