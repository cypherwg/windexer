use anyhow::{Result, anyhow};
use async_trait::async_trait;
use cid::Cid;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tracing::{info, error, instrument};

use crate::storage::{Database, CompressedAccount, CompressedBlock, CompressedTransaction};

const ACCOUNT_PREFIX: &str = "account:";
const BLOCK_PREFIX: &str = "block:";
const TRANSACTION_PREFIX: &str = "tx:";
const LAST_SLOT_KEY: &str = "last_processed_slot";

pub struct FilecoinStorage {
    ipfs_client: IpfsClient,
    cache: RwLock<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
struct CachedData<T> {
    data: T,
    cid: String,
}

impl FilecoinStorage {
    pub fn new() -> Self {
        Self {
            ipfs_client: IpfsClient::default(),
            cache: RwLock::new(HashMap::new()),
        }
    }

    #[instrument(skip(self, data))]
    async fn store<T: Serialize>(&self, key: &str, data: &T) -> Result<String> {
        let serialized = bincode::serialize(data)?;
        let res = self.ipfs_client.add(serialized).await?;
        let cid = res.hash;

        // Update cache
        let mut cache = self.cache.write().await;
        cache.insert(key.to_string(), cid.clone());

        info!("Stored data with key: {}, CID: {}", key, cid);
        Ok(cid)
    }

    #[instrument(skip(self))]
    async fn retrieve<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<T> {
        let cache = self.cache.read().await;
        if let Some(cid) = cache.get(key) {
            drop(cache); // Release the read lock
            return self.retrieve_by_cid(cid).await;
        }
        drop(cache);

        error!("Data not found in cache for key: {}", key);
        Err(anyhow!("Data not found for key: {}", key))
    }

    #[instrument(skip(self))]
    async fn retrieve_by_cid<T: for<'de> Deserialize<'de>>(&self, cid: &str) -> Result<T> {
        let cid = Cid::try_from(cid)?;
        let data = self.ipfs_client.cat(&cid.to_string()).await?;
        let deserialized: T = bincode::deserialize(&data)?;
        Ok(deserialized)
    }
}

#[async_trait]
impl Database for FilecoinStorage {
    #[instrument(skip(self, account))]
    async fn insert_compressed_account(&self, account: &CompressedAccount) -> Result<()> {
        let key = format!("{}{}", ACCOUNT_PREFIX, hex::encode(&account.pubkey));
        let cid = self.store(&key, account).await?;
        info!("Inserted compressed account with key: {}, CID: {}", key, cid);
        Ok(())
    }

    #[instrument(skip(self))]
    async fn get_compressed_account(&self, pubkey: &[u8]) -> Result<CompressedAccount> {
        let key = format!("{}{}", ACCOUNT_PREFIX, hex::encode(pubkey));
        self.retrieve(&key).await
    }

    #[instrument(skip(self, block))]
    async fn insert_compressed_block(&self, block: &CompressedBlock) -> Result<()> {
        let key = format!("{}{}", BLOCK_PREFIX, block.slot);
        let cid = self.store(&key, block).await?;
        info!("Inserted compressed block with key: {}, CID: {}", key, cid);
        Ok(())
    }

    #[instrument(skip(self))]
    async fn get_compressed_block(&self, slot: u64) -> Result<CompressedBlock> {
        let key = format!("{}{}", BLOCK_PREFIX, slot);
        self.retrieve(&key).await
    }

    #[instrument(skip(self, transaction))]
    async fn insert_compressed_transaction(&self, transaction: &CompressedTransaction) -> Result<()> {
        let key = format!("{}{}", TRANSACTION_PREFIX, hex::encode(&transaction.signature));
        let cid = self.store(&key, transaction).await?;
        info!("Inserted compressed transaction with key: {}, CID: {}", key, cid);
        Ok(())
    }

    #[instrument(skip(self))]
    async fn get_compressed_transaction(&self, signature: &[u8]) -> Result<CompressedTransaction> {
        let key = format!("{}{}", TRANSACTION_PREFIX, hex::encode(signature));
        self.retrieve(&key).await
    }

    #[instrument(skip(self))]
    async fn get_last_processed_slot(&self) -> Result<u64> {
        let cached_data: CachedData<u64> = self.retrieve(LAST_SLOT_KEY).await?;
        Ok(cached_data.data)
    }

    #[instrument(skip(self))]
    async fn update_last_processed_slot(&self, slot: u64) -> Result<()> {
        let cached_data = CachedData {
            data: slot,
            cid: "".to_string(), // This will be updated by the store method
        };
        let cid = self.store(LAST_SLOT_KEY, &cached_data).await?;
        info!("Updated last processed slot: {}, CID: {}", slot, cid);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::models::*;

    #[tokio::test]
    async fn test_filecoin_storage() {
        let storage = FilecoinStorage::new();

        let account = CompressedAccount {
            pubkey: vec![1, 2, 3, 4],
            lamports: 1000,
            owner: vec![5, 6, 7, 8],
            executable: false,
            rent_epoch: 0,
            data: vec![9, 10, 11, 12],
            proof: vec![13, 14, 15, 16],
        };
        storage.insert_compressed_account(&account).await.unwrap();
        let retrieved_account = storage.get_compressed_account(&account.pubkey).await.unwrap();
        assert_eq!(account.lamports, retrieved_account.lamports);

        let block = CompressedBlock {
            slot: 12345,
            blockhash: "abc123".to_string(),
            previous_blockhash: "def456".to_string(),
            parent_slot: 12344,
            transactions: 10,
            data: vec![17, 18, 19, 20],
            proof: vec![21, 22, 23, 24],
        };
        storage.insert_compressed_block(&block).await.unwrap();
        let retrieved_block = storage.get_compressed_block(block.slot).await.unwrap();
        assert_eq!(block.blockhash, retrieved_block.blockhash);

        let transaction = CompressedTransaction {
            signature: vec![25, 26, 27, 28],
            data: vec![29, 30, 31, 32],
            proof: vec![33, 34, 35, 36],
        };
        storage.insert_compressed_transaction(&transaction).await.unwrap();
        let retrieved_transaction = storage.get_compressed_transaction(&transaction.signature).await.unwrap();
        assert_eq!(transaction.data, retrieved_transaction.data);

        storage.update_last_processed_slot(12345).await.unwrap();
        let last_slot = storage.get_last_processed_slot().await.unwrap();
        assert_eq!(last_slot, 12345);
    }
}