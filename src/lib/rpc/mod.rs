use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

pub struct SolanaRpcClient {
    client: RpcClient,
}

impl SolanaRpcClient {
    pub fn new(url: &str) -> Self {
        Self {
            client: RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed()),
        }
    }
}