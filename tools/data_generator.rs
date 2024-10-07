use rand::Rng;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::str::FromStr;
use windexer::storage::{Storage, Account, Transaction};

fn main() {
    let storage = Storage::new("scylla://localhost:9042/windexer").unwrap();
    let mut rng = rand::thread_rng();

    for _ in 0..1000 {
        let account = Account {
            pubkey: Keypair::new().pubkey().to_string(),
            lamports: rng.gen_range(0..1000000),
            owner: Pubkey::new_unique().to_string(),
            executable: false,
            rent_epoch: rng.gen(),
            data: vec![0; rng.gen_range(0..1000)],
        };
        storage.insert_account(&account).unwrap();
    }

    for _ in 0..1000 {
        let transaction = Transaction {
            signature: Keypair::new().pubkey().to_string(),
            slot: rng.gen(),
            err: None,
            memo: Some("Sample transaction".to_string()),
            block_time: Some(chrono::Utc::now().timestamp() as u64),
        };
        storage.insert_transaction(&transaction).unwrap();
    }

    println!("Sample data generation complete.");
}