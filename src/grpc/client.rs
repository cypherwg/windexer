use tonic::transport::Channel;
use windexer_proto::windexer_client::WindexerClient;
use windexer_proto::{GetAccountRequest, GetAccountResponse};

pub struct GrpcClient {
    client: WindexerClient<Channel>,
}

impl GrpcClient {
    pub async fn new(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(addr)?.connect().await?;
        let client = WindexerClient::new(channel);
        Ok(Self { client })
    }

    pub async fn get_account(
        &mut self,
        pubkey: String,
    ) -> Result<GetAccountResponse, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(GetAccountRequest { pubkey });
        let response = self.client.get_account(request).await?;
        Ok(response.into_inner())
    }
}
