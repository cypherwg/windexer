use async_trait::async_trait;
use anyhow::Result;
use scylla::{Session, SessionBuilder};
use crate::storage::{Database, CompressedAccount, CompressedBlock, CompressedTransaction};

pub struct ScyllaStorage {
    session: Session,
}

impl ScyllaStorage {
    pub async fn new(uri: &str) -> Result<Self> {
        let session = SessionBuilder::new().known_node(uri).build().await?;
        Ok(Self { session })
    }
}

#[async_trait]
impl Database for ScyllaStorage {
    async fn insert_compressed_account(&self, account: &CompressedAccount) -> Result<()> {
        self.session
            .query(
                "INSERT INTO compressed_accounts (pubkey, lamports, owner, executable, rent_epoch, data, proof) \
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
                (
                    &account.pubkey,
                    account.lamports,
                    &account.owner,
                    account.executable,
                    account.rent_epoch,
                    &account.data,
                    &account.proof,
                ),
            )
            .await?;
        Ok(())
    }

    async fn get_compressed_account(&self, pubkey: &[u8]) -> Result<CompressedAccount> {
        let result = self.session
            .query("SELECT * FROM compressed_accounts WHERE pubkey = ?", (pubkey,))
            .await?
            .first_row()?;
        Ok(CompressedAccount {
            pubkey: result.get("pubkey")?,
            lamports: result.get("lamports")?,
            owner: result.get("owner")?,
            executable: result.get("executable")?,
            rent_epoch: result.get("rent_epoch")?,
            data: result.get("data")?,
            proof: result.get("proof")?,
        })
    }

    async fn insert_compressed_block(&self, block: &CompressedBlock) -> Result<()> {
        self.session
            .query(
                "INSERT INTO compressed_blocks (slot, blockhash, previous_blockhash, parent_slot, transactions, data, proof) \
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
                (
                    block.slot,
                    &block.blockhash,
                    &block.previous_blockhash,
                    block.parent_slot,
                    block.transactions,
                    &block.data,
                    &block.proof,
                ),
            )
            .await?;
        Ok(())
    }

    async fn get_compressed_block(&self, slot: u64) -> Result<CompressedBlock> {
        let result = self.session
            .query("SELECT * FROM compressed_blocks WHERE slot = ?", (slot,))
            .await?
            .first_row()?;
        Ok(CompressedBlock {
            slot: result.get("slot")?,
            blockhash: result.get("blockhash")?,
            previous_blockhash: result.get("previous_blockhash")?,
            parent_slot: result.get("parent_slot")?,
            transactions: result.get("transactions")?,
            data: result.get("data")?,
            proof: result.get("proof")?,
        })
    }

    async fn insert_compressed_transaction(&self, transaction: &CompressedTransaction) -> Result<()> {
        self.session
            .query(
                "INSERT INTO compressed_transactions (signature, data, proof) VALUES (?, ?, ?)",
                (&transaction.signature, &transaction.data, &transaction.proof),
            )
            .await?;
        Ok(())
    }

    async fn get_compressed_transaction(&self, signature: &[u8]) -> Result<CompressedTransaction> {
        let result = self.session
            .query("SELECT * FROM compressed_transactions WHERE signature = ?", (signature,))
            .await?
            .first_row()?;
        Ok(CompressedTransaction {
            signature: result.get("signature")?,
            data: result.get("data")?,
            proof: result.get("proof")?,
        })
    }

    async fn get_last_processed_slot(&self) -> Result<u64> {
        let result = self.session
            .query("SELECT MAX(slot) as last_slot FROM compressed_blocks", ())
            .await?
            .first_row()?;
        Ok(result.get("last_slot")?.unwrap_or(0))
    }

    async fn update_last_processed_slot(&self, slot: u64) -> Result<()> {
        self.session
            .query(
                "INSERT INTO indexer_state (key, value) VALUES ('last_processed_slot', ?) ON CONFLICT (key) DO UPDATE SET value = ?",
                (slot, slot),
            )
            .await?;
        Ok(())
    }
}