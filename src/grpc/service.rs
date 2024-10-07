use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use tonic::{Request, Response, Status};
use windexer_proto::windexer_server::Windexer;
use windexer_proto::{GetAccountRequest, GetAccountResponse};

pub struct WindexerService {
    rpc_client: RpcClient,
}

impl WindexerService {
    pub fn new(rpc_url: String) -> Self {
        let rpc_client = RpcClient::new(rpc_url);
        Self { rpc_client }
    }
}

#[tonic::async_trait]
impl Windexer for WindexerService {
    async fn get_account(
        &self,
        request: Request<GetAccountRequest>,
    ) -> Result<Response<GetAccountResponse>, Status> {
        let pubkey = request.into_inner().pubkey;
        let pubkey =
            Pubkey::from_str(&pubkey).map_err(|e| Status::invalid_argument(e.to_string()))?;

        let account = self
            .rpc_client
            .get_account(&pubkey)
            .map_err(|e| Status::internal(e.to_string()))?;

        let response = GetAccountResponse {
            lamports: account.lamports,
            owner: account.owner.to_string(),
            executable: account.executable,
            rent_epoch: account.rent_epoch,
            data: account.data,
        };

        Ok(Response::new(response))
    }
}
