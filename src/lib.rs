pub mod api;
pub mod compression;
pub mod grpc;
pub mod indexer;
pub mod metrics;
pub mod proto;
pub mod storage;
pub mod utils;
pub mod wasm;

use anyhow::Result;
use std::sync::Arc;
use tokio;

pub async fn run() -> Result<()> {
    utils::logging::init_logger()?;
    let config = utils::config::load_config()?;

    let storage = Arc::new(storage::database::Database::new(&config.database_url).await?);

    let grpc_server = grpc::server::GrpcServer::new(&config.solana_rpc_url);

    let grpc_client = grpc::client::GrpcClient::new(&config.grpc_server_url).await?;

    let indexer = indexer::Indexer::new(Arc::clone(&storage), grpc_client.clone());

    let wasm_runtime = Arc::new(wasm::WasmRuntime::new());

    let api_server = api::start_server(config.api_port, Arc::clone(&storage))?;

    let metrics_server = metrics::start_server(config.metrics_port)?;

    let grpc_server_handle = tokio::spawn(grpc_server.run(&config.grpc_server_url));

    tokio::select! {
        result = indexer.run() => {
            if let Err(e) = result {
                eprintln!("Indexer error: {:?}", e);
            } else {
                println!("Indexer finished");
            }
        },
        result = api_server => {
            if let Err(e) = result {
                eprintln!("API server error: {:?}", e);
            } else {
                println!("API server finished");
            }
        },
        result = metrics_server => {
            if let Err(e) = result {
                eprintln!("Metrics server error: {:?}", e);
            } else {
                println!("Metrics server finished");
            }
        },
        result = grpc_server_handle => {
            if let Err(e) = result {
                eprintln!("gRPC server error: {:?}", e);
            } else {
                println!("gRPC server finished");
            }
        },
    }

    Ok(())
}
