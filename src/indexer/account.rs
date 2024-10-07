use crate::storage::database::Database;
use crate::compression::Groth16Prover;
use crate::storage::models::CompressedAccount;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::account::Account;
use log::info;

pub async fn index_account(
    db: &Database,
    compressor: &Groth16Prover,
    pubkey: &Pubkey,
    account: &Account,
) -> anyhow::Result<()> {
    info!("Indexing account: {}", pubkey);

    let account_data = bincode::serialize(&account)?;
    let compressed_data = compressor.compress(&account_data)?;
    
    let proof = compressor.prove(&compressed_data)?;

    let compressed_account = CompressedAccount {
        pubkey: pubkey.to_bytes(),
        lamports: account.lamports,
        owner: account.owner.to_bytes(),
        executable: account.executable,
        rent_epoch: account.rent_epoch,
        data: compressed_data,
        proof: bincode::serialize(&proof)?,
    };

    db.insert_compressed_account(&compressed_account).await?;

    Ok(())
}

pub async fn get_compressed_account(
    db: &Database,
    compressor: &Groth16Prover,
    pubkey: &Pubkey,
) -> anyhow::Result<Account> {
    let compressed_account = db.get_compressed_account(pubkey).await?;
    let proof: ark_groth16::Proof<ark_bls12_381::Bls12_381> = bincode::deserialize(&compressed_account.proof)?;

    if !compressor.verify(&proof, &compressed_account.data)? {
        anyhow::bail!("Invalid proof for account: {}", pubkey);
    }

    let decompressed_data = compressor.decompress(&compressed_account.data)?;
    let account: Account = bincode::deserialize(&decompressed_data)?;

    Ok(account)
}