use crate::grpc::methods::*;
use crate::proto::windexer_client::WindexerClient;
use crate::proto::{
    GetAccountRequest, GetCompressedAccountRequest, GetCompressedBalanceRequest,
    GetCompressedTokenAccountBalanceRequest, GetCompressedTokenAccountsByOwnerRequest,
    GetSlotRequest, GetTransactionRequest,
};
use anyhow::Result;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use tonic::transport::Channel;

pub struct GrpcClient {
    inner: WindexerClient<Channel>,
}

impl GrpcClient {
    pub async fn new(addr: String) -> Result<Self> {
        let channel = Channel::from_shared(addr)?.connect().await?;
        let inner = WindexerClient::new(channel);
        Ok(Self { inner })
    }

    pub async fn get_slot(&mut self) -> Result<u64> {
        let request = tonic::Request::new(GetSlotRequest {});
        let response = self.inner.get_slot(request).await?;
        Ok(response.into_inner().slot)
    }

    pub async fn get_account(&mut self, pubkey: &Pubkey) -> Result<CompressedAccount> {
        let request = tonic::Request::new(GetAccountRequest {
            pubkey: pubkey.to_string(),
        });
        let response = self.inner.get_account(request).await?;
        Ok(response.into_inner().into())
    }

    pub async fn get_compressed_account(&mut self, pubkey: &Pubkey) -> Result<CompressedAccount> {
        let request = tonic::Request::new(GetCompressedAccountRequest {
            pubkey: pubkey.to_string(),
        });
        let response = self.inner.get_compressed_account(request).await?;
        Ok(response.into_inner().into())
    }

    pub async fn get_compressed_balance(&mut self, pubkey: &Pubkey) -> Result<u64> {
        let request = tonic::Request::new(GetCompressedBalanceRequest {
            pubkey: pubkey.to_string(),
        });
        let response = self.inner.get_compressed_balance(request).await?;
        Ok(response.into_inner().balance)
    }

    pub async fn get_compressed_token_account_balance(
        &mut self,
        pubkey: &Pubkey,
    ) -> Result<TokenAccountBalance> {
        let request = tonic::Request::new(GetCompressedTokenAccountBalanceRequest {
            pubkey: pubkey.to_string(),
        });
        let response = self
            .inner
            .get_compressed_token_account_balance(request)
            .await?;
        Ok(response.into_inner().into())
    }

    pub async fn get_compressed_token_accounts_by_owner(
        &mut self,
        owner: &Pubkey,
        filter: Option<RpcTokenAccountsFilter>,
    ) -> Result<Vec<CompressedTokenAccount>> {
        let filter = filter.map(|f| f.into());
        let request = tonic::Request::new(GetCompressedTokenAccountsByOwnerRequest {
            owner: owner.to_string(),
            filter: filter.map(|f| f.into()),
        });
        let response = self
            .inner
            .get_compressed_token_accounts_by_owner(request)
            .await?;
        Ok(response
            .into_inner()
            .accounts
            .into_iter()
            .map(|a| a.into())
            .collect())
    }

    pub async fn get_transaction(
        &mut self,
        signature: &Signature,
    ) -> Result<TransactionWithCompressionInfo> {
        let request = tonic::Request::new(GetTransactionRequest {
            signature: signature.to_string(),
        });
        let response = self.inner.get_transaction(request).await?;
        Ok(response.into_inner().into())
    }
}
