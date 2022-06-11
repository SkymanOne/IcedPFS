
use super::models::{BandwidthStats, BandwidthStatsParams};
use super::{Client, ClientError};


impl Client {
    /// Simple operation to list files in IPFS MFS
    pub async fn list_files(&self) -> Result<String, ClientError> {
        let response = self
            .http_client
            .post(format!("{}/files/ls", &self.config.base_address))
            .send()
            .await
            .map_err(ClientError::ApiError)?;
        response.json().await.map_err(ClientError::ApiError)
    }

    pub async fn bw_stats(&self, params: Option<BandwidthStatsParams>) -> Result<BandwidthStats, ClientError> {
        let mut url = format!("{}/stats/bw?", &self.config.base_address);
        if let Some(params) = params {
            let params = serde_urlencoded::to_string(params).map_err(ClientError::ObjectSerializationError)?;
            url.push_str(&params);
        }
        let response = self
            .http_client
            .post(url)
            .send()
            .await
            .map_err(ClientError::ApiError)?;
        response.json().await.map_err(ClientError::ApiError)
    }
    
}
