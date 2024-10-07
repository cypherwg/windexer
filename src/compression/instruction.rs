use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub enum InstructionType {
    InitTree {
        max_depth: u32,
        max_buffer_size: u32,
    },
    UpdateAccount {
        root: [u8; 32],
        previous_account: [u8; 32],
        new_account: [u8; 32],
        index: u32,
    },
    VerifyAccount {
        root: [u8; 32],
        account: [u8; 32],
        index: u32,
    },
    AppendAccount {
        account: [u8; 32],
    },
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub program_id: Pubkey,
    pub accounts: Vec<Pubkey>,
    pub data: InstructionType,
}

impl Instruction {
    pub fn new(program_id: Pubkey, accounts: Vec<Pubkey>, data: InstructionType) -> Self {
        Instruction {
            program_id,
            accounts,
            data,
        }
    }
}
