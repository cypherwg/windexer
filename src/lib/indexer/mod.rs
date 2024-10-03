use crate::rpc::SolanaRpcClient;
use crate::storage::Storage;

pub struct Indexer {
    rpc_client: SolanaRpcClient,
    storage: Storage,
}

impl Indexer {
    pub fn new(rpc_client: SolanaRpcClient, storage: Storage) -> Self {
        Self { rpc_client, storage }
    }

    pub async fn index_block(&self, slot: u64) -> anyhow::Result<()> {
        Ok(())
    }
}