use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressedAccount {
    pub pubkey: Vec<u8>,
    pub lamports: i64,
    pub owner: Vec<u8>,
    pub executable: bool,
    pub rent_epoch: i64,
    pub data: Vec<u8>,
    pub proof: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressedBlock {
    pub slot: u64,
    pub blockhash: String,
    pub previous_blockhash: String,
    pub parent_slot: u64,
    pub transactions: u64,
    pub data: Vec<u8>,
    pub proof: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompressedTransaction {
    pub signature: Vec<u8>,
    pub data: Vec<u8>,
    pub proof: Vec<u8>,
}