pub mod account;
pub mod groth16;
pub mod instruction;
pub mod merkle;
pub mod poseidon;
pub mod zk_proof;

pub use account::CompressedAccount;
pub use groth16::{generate_proof, verify_proof};
pub use instruction::{Instruction, InstructionType};
pub use merkle::MerkleTree;
pub use poseidon::poseidon_hash;
pub use zk_proof::{Proof, VerifyingKey};

pub trait Compressor {
    fn compress(&self, data: &[u8]) -> anyhow::Result<CompressedAccount>;
    fn decompress(&self, compressed: &CompressedAccount) -> anyhow::Result<Vec<u8>>;
}
