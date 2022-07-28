use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BandwidthStats {
    pub rate_in: f64,
    pub rate_out: f64,
    pub total_in: i64,
    pub total_out: i64,
}

impl ToString for BandwidthStats {
    fn to_string(&self) -> String {
        format!(
            "{:.2}b - {:.2}b - {}b - {}b",
            self.rate_in, self.rate_out, self.total_in, self.total_out
        )
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct FilesList {
    pub entries: Vec<FileEntry>,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "PascalCase")]
pub struct FileEntry {
    pub hash: String,
    pub name: String,
    pub size: i64,
    #[serde(rename = "Type")]
    pub file_type: isize,
}

impl FileEntry {

    /// Returns the shortened filename.
    /// It preserves the file extension and keeps the number of chars specified by `limit`
    /// 
    /// Example
    /// 
    /// "screenshot.png" with limit 13 will be shortened to "scr...t.png"
    pub fn get_short_filename(&self, limit: usize) -> String {
        if self.name.len() < limit {
            self.name.clone()
        } else {
            let name = self.name.clone();
            let extension: Vec<&str> = name.split('.').collect();
            let n = 13 - extension[1].len() - 4;
            let first = n / 2;
            let first = &extension[0][0..first];
            let second = extension[0].len() - n / 2 + n % 2;
            let second = &extension[0][second..];

            format!("{}...{}.{}", &first, &second, &extension[1])
        }
    }
}

/// Empty model that is returned whenever the response doesn't have a body
#[derive(Debug, Clone, Default, Deserialize)]
pub struct OkResponse;
