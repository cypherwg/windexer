use anyhow::Result;
use cid::Cid;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};

pub struct FilecoinStorage {
    ipfs_client: IpfsClient,
}

impl FilecoinStorage {
    pub fn new() -> Result<Self> {
        let ipfs_client = IpfsClient::default();
        Ok(Self { ipfs_client })
    }

    pub async fn store(&self, data: &[u8]) -> Result<String> {
        let res = self.ipfs_client.add(data).await?;
        Ok(res.hash)
    }

    pub async fn retrieve(&self, cid: &str) -> Result<Vec<u8>> {
        let cid = Cid::try_from(cid)?;
        let data = self.ipfs_client.cat(&cid.to_string()).await?;
        Ok(data)
    }
}