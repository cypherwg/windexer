use cypher_windexer::{Settings, rpc, storage, indexer, api, metrics};
use tracing::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    metrics_exporter_prometheus::PrometheusBuilder::new().install()?;

    info!("Starting Cypher Windexer");

    let settings = Settings::new()?;
    let rpc_client = rpc::SolanaRpcClient::new(&settings.solana.rpc_url);
    let storage = storage::create_storage(&settings.database)?;
    let indexer = indexer::Indexer::new(rpc_client, storage.clone());

    tokio::spawn(api::run_api_server(storage));

    loop {
        if let Err(e) = indexer.index_next_block().await {
            error!("Error indexing block: {:?}", e);
        }
        metrics::increment_indexed_blocks();
    }
}