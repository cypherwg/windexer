mod account;
mod block;
mod transaction;

use crate::storage::database::Database;
use crate::rpc::client::RpcClient;
use tokio::time::{interval, Duration};

pub struct Indexer {
    db: Database,
    rpc: RpcClient,
}

impl Indexer {
    pub fn new(db: Database, rpc: RpcClient) -> Self {
        Self { db, rpc }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            self.process_new_blocks().await?;
        }
    }

    async fn process_new_blocks(&self) -> anyhow::Result<()> {
        // Implementation details
        Ok(())
    }
}