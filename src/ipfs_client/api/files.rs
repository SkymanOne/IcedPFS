use crate::ipfs_client::models::{FilesList, OkResponse};
use serde::Serialize;

use super::ApiRoute;

#[derive(Clone, Debug, Serialize)]
pub struct ListDirsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "long")]
    use_long: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sorted: Option<bool>,
}

impl ApiRoute<FilesList> for ListDirsRequest {
    fn get_route(&self) -> &str {
        "/files/ls"
    }
}

#[allow(dead_code)]
impl ListDirsRequest {
    pub fn new() -> Self {
        Self {
            path: None,
            use_long: None,
            sorted: None,
        }
    }

    pub fn for_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    pub fn long_listed(mut self) -> Self {
        self.use_long = Some(true);
        self
    }

    pub fn sorted(mut self) -> Self {
        self.sorted = Some(true);
        self
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct WriteRequest {
    #[serde(rename = "arg")]
    path: String,

    create: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    sorted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    parents: Option<bool>,
}

impl ApiRoute<OkResponse> for WriteRequest {
    fn get_route(&self) -> &str {
        "/files/write"
    }
}

impl WriteRequest {
    pub fn new(path: String) -> Self {
        WriteRequest {
            path,
            create: false,
            sorted: None,
            parents: None,
        }
    }

    pub fn create(mut self) -> Self {
        self.create = true;
        self
    }
}
