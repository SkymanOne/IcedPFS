use std::future::Future;
use std::pin::Pin;

use super::models::{BandwidthStats, BandwidthStatsParams};
use super::{Client, ClientError};

/*
TODO: move to the separate module
TODO: build Request Builder for form a request to the IPFS server
*/
pub type FutureResult<U> = Pin<Box<dyn Future<Output = Result<U, ClientError>> + Send>>;
pub trait ApiRequest<T, U> {
    fn request(
        &self,
        params: Option<T>,
    ) -> FutureResult<U>;
}

impl Client {
    pub fn bw_stats(
        &self,
        params: Option<BandwidthStatsParams>,
    ) -> impl Future<Output = Result<BandwidthStats, ClientError>> {
        let client = self.clone();
        async move {
            let mut url = format!("{}/stats/bw?", client.config.base_address);
            if let Some(params) = params {
                let params = serde_urlencoded::to_string(params)
                    .map_err(ClientError::ObjectSerializationError)?;
                url.push_str(&params);
            }
            let response = client
                .http_client
                .post(url)
                .send()
                .await
                .map_err(ClientError::ApiError)?;
            response.json().await.map_err(ClientError::ApiError)
        }
    }
}

impl ApiRequest<BandwidthStatsParams, BandwidthStats> for Client {
    fn request(
        &self,
        params: Option<BandwidthStatsParams>,
    ) -> FutureResult<BandwidthStats> {
        Box::pin(self.bw_stats(params))
    }
}
