use crate::storage::database::Database;
use crate::rpc::client::RpcClient;
use crate::compression::Groth16Prover;
use crate::storage::models::CompressedBlock;
use solana_sdk::clock::Slot;
use solana_sdk::hash::Hash;
use solana_transaction_status::EncodedConfirmedBlock;
use log::info;

pub async fn index_block(
    db: &Database,
    rpc: &RpcClient,
    compressor: &Groth16Prover,
    block: &EncodedConfirmedBlock,
) -> anyhow::Result<()> {
    info!("Indexing block at slot: {}", block.blockhash);

    let block_data = bincode::serialize(&block)?;
    let compressed_data = compressor.compress(&block_data)?;
    
    let proof = compressor.prove(&compressed_data)?;

    let compressed_block = CompressedBlock {
        slot: block.slot,
        blockhash: block.blockhash.to_string(),
        previous_blockhash: block.previous_blockhash.to_string(),
        parent_slot: block.parent_slot,
        transactions: block.transactions.len() as u64,
        data: compressed_data,
        proof: bincode::serialize(&proof)?,
    };

    db.insert_compressed_block(&compressed_block).await?;

    Ok(())
}

pub async fn get_compressed_block(
    db: &Database,
    compressor: &Groth16Prover,
    slot: Slot,
) -> anyhow::Result<EncodedConfirmedBlock> {
    let compressed_block = db.get_compressed_block(slot).await?;
    let proof: ark_groth16::Proof<ark_bls12_381::Bls12_381> = bincode::deserialize(&compressed_block.proof)?;

    if !compressor.verify(&proof, &compressed_block.data)? {
        anyhow::bail!("Invalid proof for block at slot: {}", slot);
    }

    let decompressed_data = compressor.decompress(&compressed_block.data)?;
    let block: EncodedConfirmedBlock = bincode::deserialize(&decompressed_data)?;

    Ok(block)
}