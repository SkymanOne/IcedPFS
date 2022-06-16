use crate::ipfs_client::{models::BandwidthStats, ApiRequest, Client, ClientError, FutureResult};

#[derive(Clone, Debug)]
pub struct BandwidthStatsRequest {
    params: String,
}

impl ApiRequest<BandwidthStats> for BandwidthStatsRequest {
    fn request(&self, client: Client) -> FutureResult<BandwidthStats> {
        let request = self.clone();

        Box::pin(async move {
            let mut url = format!("{}/stats/bw?", client.config.base_address);

            if !request.params.is_empty() {
                url.push_str(&request.params)
            }

            let response = client
                .http_client
                .post(url)
                .send()
                .await
                .map_err(ClientError::ApiError)?;
            response.json().await.map_err(ClientError::ApiError)
        })
    }
}

impl BandwidthStatsRequest {
    pub fn new() -> Self {
        Self {
            params: String::new(),
        }
    }

    pub fn to_peer(mut self, peer: &str) -> Self {
        self.params.push_str(&format!("&peer={}", peer));
        self
    }
    pub fn with_protocol(mut self, proto: &str) -> Self {
        self.params.push_str(&format!("&proto={}", proto));
        self
    }
}
