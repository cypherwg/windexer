use crate::compression::poseidon::poseidon_hash;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct CompressedAccount {
    pub pubkey: Pubkey,
    pub lamports: u64,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
    pub data: Vec<u8>,
    pub commitment: [u8; 32],
}

impl CompressedAccount {
    pub fn new(
        pubkey: Pubkey,
        lamports: u64,
        owner: Pubkey,
        executable: bool,
        rent_epoch: u64,
        data: Vec<u8>,
    ) -> Self {
        let commitment =
            Self::compute_commitment(&pubkey, lamports, &owner, executable, rent_epoch, &data);
        CompressedAccount {
            pubkey,
            lamports,
            owner,
            executable,
            rent_epoch,
            data,
            commitment,
        }
    }

    fn compute_commitment(
        pubkey: &Pubkey,
        lamports: u64,
        owner: &Pubkey,
        executable: bool,
        rent_epoch: u64,
        data: &[u8],
    ) -> [u8; 32] {
        let mut input = Vec::new();
        input.extend_from_slice(&pubkey.to_bytes());
        input.extend_from_slice(&lamports.to_le_bytes());
        input.extend_from_slice(&owner.to_bytes());
        input.push(executable as u8);
        input.extend_from_slice(&rent_epoch.to_le_bytes());
        input.extend_from_slice(data);

        poseidon_hash(&[input.as_slice()])
    }

    pub fn verify_commitment(&self) -> bool {
        let computed_commitment = Self::compute_commitment(
            &self.pubkey,
            self.lamports,
            &self.owner,
            self.executable,
            self.rent_epoch,
            &self.data,
        );
        computed_commitment == self.commitment
    }
}
