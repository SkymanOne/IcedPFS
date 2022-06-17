use crate::ipfs_client::models::BandwidthStats;
use serde::Serialize;

use super::ApiRoute;

#[derive(Clone, Debug, Serialize)]
pub struct BandwidthStatsRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    proto: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    peer: Option<&'a str>,
}

impl<'a> ApiRoute<BandwidthStats> for BandwidthStatsRequest<'a> {
    fn get_route(&self) -> &str {
        "/stats/bw"
    }
}

impl<'a> BandwidthStatsRequest<'a> {
    pub fn new() -> Self {
        Self {
            proto: None,
            peer: None,
        }
    }

    pub fn with_peer(mut self, peer: &'a str) -> Self {
        self.peer = Some(peer);
        self
    }

    pub fn with_protocol(mut self, proto: &'a str) -> Self {
        self.peer = Some(proto);
        self
    }
}
