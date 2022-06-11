use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct BandwidthStats {
    #[serde(rename = "RateIn")]
    pub rate_in: f64,
    #[serde(rename = "RateOut")]
    pub rate_out: f64,
    #[serde(rename = "TotalIn")]
    pub total_in: i64,
    #[serde(rename = "TotalOut")]
    pub total_out: i64
}

impl ToString for BandwidthStats {
    fn to_string(&self) -> String {
        format!("{:.2}b - {:.2}b - {}b - {}b", self.rate_in, self.rate_out, self.total_in, self.total_out)
    }
}

#[derive(Serialize, Default)]
pub struct BandwidthStatsParams {
    pub peer: String,
    pub proto: String,
}