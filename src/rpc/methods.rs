use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressedAccount {
    pub pubkey: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAccountBalance {
    pub amount: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressedTokenAccount {
    pub pubkey: Pubkey,
    pub account: CompressedAccount,
    pub amount: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionWithCompressionInfo {
    pub transaction: EncodedConfirmedTransaction,
    pub compression_info: CompressionInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressionInfo {
    pub compressed_accounts: Vec<CompressedAccount>,
    pub decompressed_accounts: Vec<CompressedAccount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RpcTokenAccountsFilter {
    Mint(Pubkey),
    ProgramId(Pubkey),
}

pub fn get_compressed_account(rpc: &RpcClient, pubkey: &Pubkey) -> Result<CompressedAccount> {
    let response: CompressedAccount = rpc.send(
        RpcRequest::Custom("getCompressedAccount"),
        json!([pubkey.to_string()]),
    )?;
    Ok(response)
}

pub fn get_compressed_balance(rpc: &RpcClient, pubkey: &Pubkey) -> Result<u64> {
    let response: u64 = rpc.send(
        RpcRequest::Custom("getCompressedBalance"),
        json!([pubkey.to_string()]),
    )?;
    Ok(response)
}

pub fn get_compressed_token_account_balance(rpc: &RpcClient, pubkey: &Pubkey) -> Result<TokenAccountBalance> {
    let response: TokenAccountBalance = rpc.send(
        RpcRequest::Custom("getCompressedTokenAccountBalance"),
        json!([pubkey.to_string()]),
    )?;
    Ok(response)
}

pub fn get_compressed_token_accounts_by_owner(
    rpc: &RpcClient,
    owner: &Pubkey,
    filter: Option<RpcTokenAccountsFilter>,
) -> Result<Vec<CompressedTokenAccount>> {
    let filter_json = match filter {
        Some(RpcTokenAccountsFilter::Mint(mint)) => json!({"mint": mint.to_string()}),
        Some(RpcTokenAccountsFilter::ProgramId(program_id)) => json!({"programId": program_id.to_string()}),
        None => json!(null),
    };

    let response: Vec<CompressedTokenAccount> = rpc.send(
        RpcRequest::Custom("getCompressedTokenAccountsByOwner"),
        json!([owner.to_string(), filter_json]),
    )?;
    Ok(response)
}

pub fn get_transaction_with_compression_info(rpc: &RpcClient, signature: &Signature) -> Result<TransactionWithCompressionInfo> {
    let response: TransactionWithCompressionInfo = rpc.send(
        RpcRequest::Custom("getTransactionWithCompressionInfo"),
        json!([signature.to_string()]),
    )?;
    Ok(response)
}