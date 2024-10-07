pub mod groth16;
pub mod merkle;
pub mod poseidon;
pub mod zk_snark;

pub use self::groth16::Groth16Prover;
pub use self::merkle::MerkleTree;
pub use self::poseidon::PoseidonHash;
pub use self::zk_snark::{Proof, VerifyingKey};

pub struct CompressedAccount {
    pub address: [u8; 32],
    pub data: Vec<u8>,
    pub proof: Proof,
}

pub trait Compressor {
    fn compress(&self, data: &[u8]) -> anyhow::Result<CompressedAccount>;
    fn decompress(&self, compressed: &CompressedAccount) -> anyhow::Result<Vec<u8>>;
}