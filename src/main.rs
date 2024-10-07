use anyhow::Result;
use windexer::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}