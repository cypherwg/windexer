use async_trait::async_trait;

#[async_trait]
pub trait Storage {
    async fn store_account(&self, pubkey: &str, data: &[u8]) -> anyhow::Result<()>;
    async fn get_account(&self, pubkey: &str) -> anyhow::Result<Option<Vec<u8>>>;
}

pub mod redis;
pub mod scylla;
pub mod clickhouse;