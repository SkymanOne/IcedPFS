use crate::ipfs_client::models::FilesList;
use serde::Serialize;

use super::ApiRoute;

#[derive(Clone, Debug, Serialize)]
pub struct ListDirsRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "long")]
    use_long: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sorted: Option<bool>,
}

impl<'a> ApiRoute<FilesList> for ListDirsRequest<'a> {
    fn get_route(&self) -> &str {
        "/files/ls"
    }
}

#[allow(dead_code)]
impl<'a> ListDirsRequest<'a> {
    pub fn new() -> Self {
        Self {
            path: None,
            use_long: None,
            sorted: None,
        }
    }

    pub fn for_path(mut self, path: &'a str) -> Self {
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
pub struct WriteRequest<'a> {
    #[serde(rename = "arg")]
    path: &'a str,

    create: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    sorted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    parents: Option<bool>,
}

impl<'a> ApiRoute<FilesList> for WriteRequest<'a> {
    fn get_route(&self) -> &str {
        "/files/write"
    }
}

impl<'a> WriteRequest<'a> {
    pub fn new(path: &'a str) -> Self {
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
