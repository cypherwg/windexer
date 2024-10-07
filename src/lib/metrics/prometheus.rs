use prometheus::{
    Encoder, IntCounter, IntGauge, Opts, Registry, TextEncoder,
    Histogram, HistogramOpts,
};
use warp::Filter;
use std::convert::Infallible;
use anyhow::Result;

pub struct PrometheusMetrics {
    registry: Registry,
    processed_blocks: IntCounter,
    processed_transactions: IntCounter,
    processed_accounts: IntCounter,
    block_processing_time: Histogram,
    transaction_processing_time: Histogram,
    account_processing_time: Histogram,
    last_processed_slot: IntGauge,
    indexer_status: IntGauge,
}

impl PrometheusMetrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let processed_blocks = IntCounter::new("processed_blocks_total", "Total number of processed blocks").unwrap();
        let processed_transactions = IntCounter::new("processed_transactions_total", "Total number of processed transactions").unwrap();
        let processed_accounts = IntCounter::new("processed_accounts_total", "Total number of processed accounts").unwrap();

        let block_processing_time = Histogram::with_opts(
            HistogramOpts::new("block_processing_time_seconds", "Time taken to process a block")
        ).unwrap();
        let transaction_processing_time = Histogram::with_opts(
            HistogramOpts::new("transaction_processing_time_seconds", "Time taken to process a transaction")
        ).unwrap();
        let account_processing_time = Histogram::with_opts(
            HistogramOpts::new("account_processing_time_seconds", "Time taken to process an account")
        ).unwrap();

        let last_processed_slot = IntGauge::new("last_processed_slot", "Last processed slot number").unwrap();
        let indexer_status = IntGauge::with_opts(
            Opts::new("indexer_status", "Indexer status (0: stopped, 1: running, 2: error)")
        ).unwrap();

        registry.register(Box::new(processed_blocks.clone())).unwrap();
        registry.register(Box::new(processed_transactions.clone())).unwrap();
        registry.register(Box::new(processed_accounts.clone())).unwrap();
        registry.register(Box::new(block_processing_time.clone())).unwrap();
        registry.register(Box::new(transaction_processing_time.clone())).unwrap();
        registry.register(Box::new(account_processing_time.clone())).unwrap();
        registry.register(Box::new(last_processed_slot.clone())).unwrap();
        registry.register(Box::new(indexer_status.clone())).unwrap();

        Self {
            registry,
            processed_blocks,
            processed_transactions,
            processed_accounts,
            block_processing_time,
            transaction_processing_time,
            account_processing_time,
            last_processed_slot,
            indexer_status,
        }
    }

    pub fn increment_processed_blocks(&self) {
        self.processed_blocks.inc();
    }

    pub fn increment_processed_transactions(&self) {
        self.processed_transactions.inc();
    }

    pub fn increment_processed_accounts(&self) {
        self.processed_accounts.inc();
    }

    pub fn observe_block_processing_time(&self, duration: f64) {
        self.block_processing_time.observe(duration);
    }

    pub fn observe_transaction_processing_time(&self, duration: f64) {
        self.transaction_processing_time.observe(duration);
    }

    pub fn observe_account_processing_time(&self, duration: f64) {
        self.account_processing_time.observe(duration);
    }

    pub fn set_last_processed_slot(&self, slot: i64) {
        self.last_processed_slot.set(slot);
    }

    pub fn set_indexer_status(&self, status: &str) {
        let status_value = match status {
            "running" => 1,
            "error" => 2,
            _ => 0,
        };
        self.indexer_status.set(status_value);
    }

    pub fn gather(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        buffer
    }
}

pub async fn start_metrics_server(port: u16) -> Result<()> {
    let metrics_route = warp::path!("metrics").and_then(serve_metrics);
    let routes = metrics_route.with(warp::cors().allow_any_origin());

    println!("Starting metrics server on port {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
    Ok(())
}

async fn serve_metrics() -> Result<impl warp::Reply, Infallible> {
    let metrics = METRICS.lock().await.gather();
    Ok(warp::reply::with_header(metrics, "Content-Type", "text/plain"))
}