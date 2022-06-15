use crate::{
    ipfs_client::{models::{BandwidthStats, BandwidthStatsParams}, Client, ClientError},
};

use super::ApiRequest;

#[derive(Clone, Debug)]
pub struct BandwidthStatsRequest {
    client: Client,
    params: String,
}

impl ApiRequest<BandwidthStatsParams, BandwidthStats> for BandwidthStatsRequest {
    fn request(&self, params: Option<BandwidthStatsParams>) -> super::FutureResult<BandwidthStats> {
        let request = self.clone();

        Box::pin(async move {
            let mut url = format!("{}/stats/bw?", request.client.config.base_address);
            // if let Some(params) = params {
            //     let params = serde_urlencoded::to_string(params)
            //         .map_err(ClientError::ObjectSerializationError)?;
            //     url.push_str(&params);
            // }

                if !request.params.is_empty() {
                    url.push_str(&request.params)
                }

            let response = request.client
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
    pub fn new(client: Client) -> Self {
        Self { client, params: String::new() }
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