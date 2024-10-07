use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use anyhow::Result;
use crate::storage::models::*;

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn insert_compressed_account(&self, account: &CompressedAccount) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO compressed_accounts (pubkey, lamports, owner, executable, rent_epoch, data, proof)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (pubkey) DO UPDATE
            SET lamports = $2, owner = $3, executable = $4, rent_epoch = $5, data = $6, proof = $7
            "#,
            account.pubkey,
            account.lamports as i64,
            account.owner,
            account.executable,
            account.rent_epoch as i64,
            account.data,
            account.proof,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_compressed_account(&self, pubkey: &[u8]) -> Result<CompressedAccount> {
        let account = sqlx::query_as!(
            CompressedAccount,
            r#"
            SELECT * FROM compressed_accounts WHERE pubkey = $1
            "#,
            pubkey,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(account)
    }

    pub async fn insert_compressed_block(&self, block: &CompressedBlock) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO compressed_blocks (slot, blockhash, previous_blockhash, parent_slot, transactions, data, proof)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (slot) DO UPDATE
            SET blockhash = $2, previous_blockhash = $3, parent_slot = $4, transactions = $5, data = $6, proof = $7
            "#,
            block.slot as i64,
            block.blockhash,
            block.previous_blockhash,
            block.parent_slot as i64,
            block.transactions as i64,
            block.data,
            block.proof,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_compressed_block(&self, slot: u64) -> Result<CompressedBlock> {
        let block = sqlx::query_as!(
            CompressedBlock,
            r#"
            SELECT * FROM compressed_blocks WHERE slot = $1
            "#,
            slot as i64,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(block)
    }

    pub async fn insert_compressed_transaction(&self, transaction: &CompressedTransaction) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO compressed_transactions (signature, data, proof)
            VALUES ($1, $2, $3)
            ON CONFLICT (signature) DO UPDATE
            SET data = $2, proof = $3
            "#,
            transaction.signature,
            transaction.data,
            transaction.proof,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_compressed_transaction(&self, signature: &[u8]) -> Result<CompressedTransaction> {
        let transaction = sqlx::query_as!(
            CompressedTransaction,
            r#"
            SELECT * FROM compressed_transactions WHERE signature = $1
            "#,
            signature,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(transaction)
    }

    pub async fn get_last_processed_slot(&self) -> Result<u64> {
        let last_slot = sqlx::query!(
            r#"
            SELECT MAX(slot) as last_slot FROM compressed_blocks
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(last_slot.last_slot.unwrap_or(0) as u64)
    }

    pub async fn update_last_processed_slot(&self, slot: u64) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO indexer_state (key, value)
            VALUES ('last_processed_slot', $1)
            ON CONFLICT (key) DO UPDATE
            SET value = $1
            "#,
            slot as i64,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}