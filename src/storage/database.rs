use async_trait::async_trait;
use anyhow::Result;
use crate::storage::models::*;

#[async_trait]
pub trait Database: Send + Sync {
    async fn insert_compressed_account(&self, account: &CompressedAccount) -> Result<()>;
    async fn get_compressed_account(&self, pubkey: &[u8]) -> Result<CompressedAccount>;
    async fn insert_compressed_block(&self, block: &CompressedBlock) -> Result<()>;
    async fn get_compressed_block(&self, slot: u64) -> Result<CompressedBlock>;
    async fn insert_compressed_transaction(&self, transaction: &CompressedTransaction) -> Result<()>;
    async fn get_compressed_transaction(&self, signature: &[u8]) -> Result<CompressedTransaction>;
    async fn get_last_processed_slot(&self) -> Result<u64>;
    async fn update_last_processed_slot(&self, slot: u64) -> Result<()>;
}