use crate::proto;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

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

impl From<proto::CompressedAccount> for CompressedAccount {
    fn from(proto_account: proto::CompressedAccount) -> Self {
        Self {
            pubkey: Pubkey::new_from_array(proto_account.pubkey.try_into().unwrap()),
            lamports: proto_account.lamports,
            data: proto_account.data,
            owner: Pubkey::new_from_array(proto_account.owner.try_into().unwrap()),
            executable: proto_account.executable,
            rent_epoch: proto_account.rent_epoch,
        }
    }
}

impl From<proto::TokenAccountBalance> for TokenAccountBalance {
    fn from(proto_balance: proto::TokenAccountBalance) -> Self {
        Self {
            amount: proto_balance.amount,
            decimals: proto_balance.decimals as u8,
        }
    }
}

impl From<proto::CompressedTokenAccount> for CompressedTokenAccount {
    fn from(proto_token_account: proto::CompressedTokenAccount) -> Self {
        Self {
            pubkey: Pubkey::new_from_array(proto_token_account.pubkey.try_into().unwrap()),
            account: proto_token_account.account.unwrap().into(),
            amount: proto_token_account.amount,
            decimals: proto_token_account.decimals as u8,
        }
    }
}

impl From<proto::TransactionWithCompressionInfo> for TransactionWithCompressionInfo {
    fn from(proto_tx: proto::TransactionWithCompressionInfo) -> Self {
        Self {
            transaction: proto_tx.transaction.unwrap().into(),
            compression_info: proto_tx.compression_info.unwrap().into(),
        }
    }
}

impl From<proto::CompressionInfo> for CompressionInfo {
    fn from(proto_info: proto::CompressionInfo) -> Self {
        Self {
            compressed_accounts: proto_info
                .compressed_accounts
                .into_iter()
                .map(|a| a.into())
                .collect(),
            decompressed_accounts: proto_info
                .decompressed_accounts
                .into_iter()
                .map(|a| a.into())
                .collect(),
        }
    }
}

impl From<RpcTokenAccountsFilter> for proto::RpcTokenAccountsFilter {
    fn from(filter: RpcTokenAccountsFilter) -> Self {
        match filter {
            RpcTokenAccountsFilter::Mint(pubkey) => proto::RpcTokenAccountsFilter {
                filter_type: Some(proto::rpc_token_accounts_filter::FilterType::Mint(
                    proto::Pubkey {
                        data: pubkey.to_bytes().to_vec(),
                    },
                )),
            },
            RpcTokenAccountsFilter::ProgramId(pubkey) => proto::RpcTokenAccountsFilter {
                filter_type: Some(proto::rpc_token_accounts_filter::FilterType::ProgramId(
                    proto::Pubkey {
                        data: pubkey.to_bytes().to_vec(),
                    },
                )),
            },
        }
    }
}
