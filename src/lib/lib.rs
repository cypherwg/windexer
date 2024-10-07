pub mod api;
pub mod compression;
pub mod indexer;
pub mod metrics;
pub mod rpc;
pub mod storage;
pub mod utils;
pub mod wasm;

use anyhow::Result;
use tokio;

pub async fn run() -> Result<()> {
    utils::logging::init_logger()?;
    let config = utils::config::load_config()?;
    
    let storage = storage::database::Database::new(&config.database_url).await?;
    let rpc_client = rpc::client::RpcClient::new(&config.solana_rpc_url);
    let indexer = indexer::Indexer::new(storage.clone(), rpc_client.clone());
    
    let api_server = api::start_server(config.api_port, storage.clone())?;
    let metrics_server = metrics::start_server(config.metrics_port)?;
    
    tokio::select! {
        _ = indexer.run() => println!("Indexer finished"),
        _ = api_server => println!("API server finished"),
        _ = metrics_server => println!("Metrics server finished"),
    }
    
    Ok(())
}