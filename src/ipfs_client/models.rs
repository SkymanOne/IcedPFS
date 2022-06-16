use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BandwidthStats {
    pub rate_in: f64,
    pub rate_out: f64,
    pub total_in: i64,
    pub total_out: i64
}

impl ToString for BandwidthStats {
    fn to_string(&self) -> String {
        format!("{:.2}b - {:.2}b - {}b - {}b", self.rate_in, self.rate_out, self.total_in, self.total_out)
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct FilesList {
    pub rate_in: f64,
    pub rate_out: f64,
    pub total_in: i64,
    pub total_out: i64
}
