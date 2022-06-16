use std::{process::Command, io::BufRead, pin::Pin, sync::Arc};

use futures::Future;

pub mod connection;
pub mod api;
pub mod models;

pub type FutureResult<U> = Pin<Box<dyn Future<Output = Result<U, ClientError>> + Send>>;

/*
TODO: move to the separate module
TODO: build Request Builder for form a request to the IPFS server
*/
pub trait ApiRequest<U> {
    fn request(
       &self, 
       client: Client
    ) -> FutureResult<U>;
}

#[derive(Clone, Debug)]
pub struct Config { 
    base_address: String
 }

#[derive(Clone, Debug)]
pub struct Client {
    http_client: reqwest::Client,
    config: Arc<Config>
}

#[derive(Debug)]
pub enum ClientError {
    NoIPFS,
    ErrorCreatingIPFS,
    ApiError(reqwest::Error),
    ObjectSerializationError(serde_urlencoded::ser::Error),
}

impl Default for Client {
    fn default() -> Self {
        //create a client
        let config = Config { base_address: format!("{}{}", "http://127.0.0.1:5001", "/api/v0") };
        let http_client = reqwest::Client::new();
        Self { http_client, config: Arc::new(config) }
    }
}


impl Client {
    pub fn make_request<T, U: ApiRequest<T>>(&self, request: U) -> FutureResult<T> {
        request.request(self.clone())
    }
}