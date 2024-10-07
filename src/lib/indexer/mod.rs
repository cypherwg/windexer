mod account;
mod block;
mod transaction;

use crate::storage::database::Database;
use crate::rpc::client::RpcClient;
use crate::compression::{Compressor, Groth16Prover};
use tokio::time::{interval, Duration};
use log::{info, error};

pub struct Indexer {
    db: Database,
    rpc: RpcClient,
    compressor: Groth16Prover,
}

impl Indexer {
    pub fn new(db: Database, rpc: RpcClient, compressor: Groth16Prover) -> Self {
        Self { db, rpc, compressor }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        info!("Starting indexer");
        let mut interval = interval(Duration::from_secs(1));
        let mut last_processed_slot = self.db.get_last_processed_slot().await?;

        loop {
            interval.tick().await;
            match self.process_new_blocks(last_processed_slot).await {
                Ok(new_last_processed_slot) => {
                    last_processed_slot = new_last_processed_slot;
                    self.db.update_last_processed_slot(last_processed_slot).await?;
                }
                Err(e) => {
                    error!("Error processing new blocks: {:?}", e);
                }
            }
        }
    }

    async fn process_new_blocks(&self, last_processed_slot: u64) -> anyhow::Result<u64> {
        let current_slot = self.rpc.get_slot().await?;
        
        for slot in (last_processed_slot + 1)..=current_slot {
            info!("Processing block at slot {}", slot);
            let block = self.rpc.get_block(slot).await?;
            
            block::index_block(&self.db, &self.rpc, &self.compressor, &block).await?;

            for transaction in block.transactions {
                transaction::index_transaction(&self.db, &self.compressor, &transaction).await?;
                
                for account_key in transaction.message.account_keys {
                    let account = self.rpc.get_account(&account_key).await?;
                    account::index_account(&self.db, &self.compressor, &account_key, &account).await?;
                }
            }
        }

        Ok(current_slot)
    }
}