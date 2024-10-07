use solana_client::rpc_client::RpcClient as SolanaRpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::Signature,
    transaction::Transaction,
};
use solana_transaction_status::UiConfirmedBlock;
use anyhow::Result;
use crate::rpc::methods::*;

pub struct RpcClient {
    inner: SolanaRpcClient,
}

impl RpcClient {
    pub fn new(url: &str) -> Self {
        Self {
            inner: SolanaRpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed()),
        }
    }

    pub async fn get_slot(&self) -> Result<u64> {
        Ok(self.inner.get_slot()?)
    }

    pub async fn get_block(&self, slot: u64) -> Result<UiConfirmedBlock> {
        Ok(self.inner.get_block_with_encoding(slot, solana_transaction_status::UiTransactionEncoding::Json)?)
    }

    pub async fn get_account(&self, pubkey: &Pubkey) -> Result<solana_sdk::account::Account> {
        Ok(self.inner.get_account(pubkey)?)
    }

    pub async fn get_transaction(&self, signature: &Signature) -> Result<EncodedConfirmedTransaction> {
        Ok(self.inner.get_transaction(signature, solana_transaction_status::UiTransactionEncoding::Json)?)
    }

    pub async fn send_transaction(&self, transaction: &Transaction) -> Result<Signature> {
        Ok(self.inner.send_transaction(transaction)?)
    }

    pub async fn get_compressed_account(&self, pubkey: &Pubkey) -> Result<CompressedAccount> {
        get_compressed_account(&self.inner, pubkey)
    }

    pub async fn get_compressed_balance(&self, pubkey: &Pubkey) -> Result<u64> {
        get_compressed_balance(&self.inner, pubkey)
    }

    pub async fn get_compressed_token_account_balance(&self, pubkey: &Pubkey) -> Result<TokenAccountBalance> {
        get_compressed_token_account_balance(&self.inner, pubkey)
    }

    pub async fn get_compressed_token_accounts_by_owner(&self, owner: &Pubkey, filter: Option<RpcTokenAccountsFilter>) -> Result<Vec<CompressedTokenAccount>> {
        get_compressed_token_accounts_by_owner(&self.inner, owner, filter)
    }

    pub async fn get_transaction_with_compression_info(&self, signature: &Signature) -> Result<TransactionWithCompressionInfo> {
        get_transaction_with_compression_info(&self.inner, signature)
    }
}