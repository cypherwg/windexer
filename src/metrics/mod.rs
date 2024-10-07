mod prometheus;

use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;

pub use self::prometheus::{PrometheusMetrics, start_metrics_server};

lazy_static! {
    pub static ref METRICS: Arc<Mutex<PrometheusMetrics>> = Arc::new(Mutex::new(PrometheusMetrics::new()));
}

pub async fn increment_processed_blocks() {
    METRICS.lock().await.increment_processed_blocks();
}

pub async fn increment_processed_transactions() {
    METRICS.lock().await.increment_processed_transactions();
}

pub async fn increment_processed_accounts() {
    METRICS.lock().await.increment_processed_accounts();
}

pub async fn observe_block_processing_time(duration: f64) {
    METRICS.lock().await.observe_block_processing_time(duration);
}

pub async fn observe_transaction_processing_time(duration: f64) {
    METRICS.lock().await.observe_transaction_processing_time(duration);
}

pub async fn observe_account_processing_time(duration: f64) {
    METRICS.lock().await.observe_account_processing_time(duration);
}

pub async fn set_last_processed_slot(slot: i64) {
    METRICS.lock().await.set_last_processed_slot(slot);
}

pub async fn set_indexer_status(status: &str) {
    METRICS.lock().await.set_indexer_status(status);
}