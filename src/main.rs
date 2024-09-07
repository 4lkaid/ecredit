use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let _worker_guard = ecredit::run().await?;
    Ok(())
}
