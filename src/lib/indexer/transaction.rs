use crate::storage::database::Database;
use crate::compression::Groth16Prover;
use crate::storage::models::CompressedTransaction;
use solana_sdk::transaction::Transaction;
use solana_sdk::signature::Signature;
use log::info;

pub async fn index_transaction(
    db: &Database,
    compressor: &Groth16Prover,
    transaction: &Transaction,
) -> anyhow::Result<()> {
    info!("Indexing transaction: {}", transaction.signatures[0]);

    let transaction_data = bincode::serialize(&transaction)?;
    let compressed_data = compressor.compress(&transaction_data)?;
    
    let proof = compressor.prove(&compressed_data)?;

    let compressed_transaction = CompressedTransaction {
        signature: transaction.signatures[0].to_bytes(),
        data: compressed_data,
        proof: bincode::serialize(&proof)?,
    };

    db.insert_compressed_transaction(&compressed_transaction).await?;

    Ok(())
}

pub async fn get_compressed_transaction(
    db: &Database,
    compressor: &Groth16Prover,
    signature: &Signature,
) -> anyhow::Result<Transaction> {
    let compressed_transaction = db.get_compressed_transaction(signature).await?;
    let proof: ark_groth16::Proof<ark_bls12_381::Bls12_381> = bincode::deserialize(&compressed_transaction.proof)?;

    if !compressor.verify(&proof, &compressed_transaction.data)? {
        anyhow::bail!("Invalid proof for transaction: {}", signature);
    }

    let decompressed_data = compressor.decompress(&compressed_transaction.data)?;
    let transaction: Transaction = bincode::deserialize(&decompressed_data)?;

    Ok(transaction)
}