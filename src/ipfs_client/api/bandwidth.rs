use crate::ipfs_client::{models::BandwidthStats};

use super::ApiRoute;

#[derive(Clone, Debug)]
pub struct BandwidthStatsRequest {
    params: String,
}

impl ApiRoute<BandwidthStats> for BandwidthStatsRequest {
    fn get_route(&self) -> String {
        return format!("/stats/bw?{}", self.params);
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
