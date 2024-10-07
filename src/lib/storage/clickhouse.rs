use async_trait::async_trait;
use anyhow::Result;
use clickhouse::{Client, Row};
use crate::storage::{Database, CompressedAccount, CompressedBlock, CompressedTransaction};

pub struct ClickHouseStorage {
    client: Client,
}

impl ClickHouseStorage {
    pub fn new(url: &str) -> Result<Self> {
        let client = Client::default()
            .with_url(url)
            .with_database("windexer");
        Ok(Self { client })
    }
}

#[async_trait]
impl Database for ClickHouseStorage {
    async fn insert_compressed_account(&self, account: &CompressedAccount) -> Result<()> {
        self.client
            .query("INSERT INTO compressed_accounts (pubkey, lamports, owner, executable, rent_epoch, data, proof) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(&account.pubkey)
            .bind(account.lamports)
            .bind(&account.owner)
            .bind(account.executable)
            .bind(account.rent_epoch)
            .bind(&account.data)
            .bind(&account.proof)
            .execute()
            .await?;
        Ok(())
    }

    async fn get_compressed_account(&self, pubkey: &[u8]) -> Result<CompressedAccount> {
        let row: Row = self.client
            .query("SELECT * FROM compressed_accounts WHERE pubkey = ?")
            .bind(pubkey)
            .fetch_one()
            .await?;
        Ok(CompressedAccount {
            pubkey: row.get("pubkey")?,
            lamports: row.get("lamports")?,
            owner: row.get("owner")?,
            executable: row.get("executable")?,
            rent_epoch: row.get("rent_epoch")?,
            data: row.get("data")?,
            proof: row.get("proof")?,
        })
    }

    async fn insert_compressed_block(&self, block: &CompressedBlock) -> Result<()> {
        self.client
            .query("INSERT INTO compressed_blocks (slot, blockhash, previous_blockhash, parent_slot, transactions, data, proof) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(block.slot)
            .bind(&block.blockhash)
            .bind(&block.previous_blockhash)
            .bind(block.parent_slot)
            .bind(block.transactions)
            .bind(&block.data)
            .bind(&block.proof)
            .execute()
            .await?;
        Ok(())
    }

    async fn get_compressed_block(&self, slot: u64) -> Result<CompressedBlock> {
        let row: Row = self.client
            .query("SELECT * FROM compressed_blocks WHERE slot = ?")
            .bind(slot)
            .fetch_one()
            .await?;
        Ok(CompressedBlock {
            slot: row.get("slot")?,
            blockhash: row.get("blockhash")?,
            previous_blockhash: row.get("previous_blockhash")?,
            parent_slot: row.get("parent_slot")?,
            transactions: row.get("transactions")?,
            data: row.get("data")?,
            proof: row.get("proof")?,
        })
    }

    async fn insert_compressed_transaction(&self, transaction: &CompressedTransaction) -> Result<()> {
        self.client
            .query("INSERT INTO compressed_transactions (signature, data, proof) VALUES (?, ?, ?)")
            .bind(&transaction.signature)
            .bind(&transaction.data)
            .bind(&transaction.proof)
            .execute()
            .await?;
        Ok(())
    }

    async fn get_compressed_transaction(&self, signature: &[u8]) -> Result<CompressedTransaction> {
        let row: Row = self.client
            .query("SELECT * FROM compressed_transactions WHERE signature = ?")
            .bind(signature)
            .fetch_one()
            .await?;
        Ok(CompressedTransaction {
            signature: row.get("signature")?,
            data: row.get("data")?,
            proof: row.get("proof")?,
        })
    }

    async fn get_last_processed_slot(&self) -> Result<u64> {
        let row: Row = self.client
            .query("SELECT MAX(slot) as last_slot FROM compressed_blocks")
            .fetch_one()
            .await?;
        Ok(row.get::<Option<u64>, _>("last_slot")?.unwrap_or(0))
    }

    async fn update_last_processed_slot(&self, slot: u64) -> Result<()> {
        self.client
            .query("INSERT INTO indexer_state (key, value) VALUES ('last_processed_slot', ?) ON CONFLICT (key) DO UPDATE SET value = ?")
            .bind(slot)
            .bind(slot)
            .execute()
            .await?;
        Ok(())
    }
}