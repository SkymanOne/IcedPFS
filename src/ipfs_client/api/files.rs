use crate::ipfs_client::models::{FileEntry, FilesList};
use serde::Serialize;

use super::ApiRoute;

#[derive(Clone, Debug, Serialize)]
pub struct ListDirsRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    use_long: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sorted: Option<bool>,
}

impl<'a> ApiRoute<FilesList> for ListDirsRequest<'a> {
    fn get_route(&self) -> &str {
        "/files/ls"
    }
}

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

    pub fn use_long(mut self) -> Self {
        self.use_long = Some(true);
        self
    }

    pub fn sorted(mut self) -> Self {
        self.sorted = Some(true);
        self
    }
}
