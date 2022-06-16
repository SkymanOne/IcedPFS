use std::{io::BufRead, pin::Pin, process::Command, sync::Arc};

use futures::Future;
use serde::de::DeserializeOwned;

use self::api::ApiRoute;

pub mod api;
pub mod connection;
pub mod models;

pub type FutureResult<U> = Pin<Box<dyn Future<Output = Result<U, ClientError>> + Send>>;

#[derive(Clone, Debug)]
pub struct Config {
    base_address: String,
}

#[derive(Clone, Debug)]
pub struct Client {
    http_client: reqwest::Client,
    config: Arc<Config>,
}

#[derive(Debug)]
pub enum ClientError {
    NoIPFS,
    ErrorCreatingIPFS,
    ApiError(reqwest::Error),
}

impl Default for Client {
    fn default() -> Self {
        //create a client
        let config = Config {
            base_address: format!("{}{}", "http://127.0.0.1:5001", "/api/v0"),
        };
        let http_client = reqwest::Client::new();
        Self {
            http_client,
            config: Arc::new(config),
        }
    }
}

impl Client {
    pub fn make_request<T, U>(&self, route: U) -> FutureResult<T>
    where
        U: ApiRoute<T> + Send + 'static,
        T: DeserializeOwned,
    {
        let client = self.clone();
        Box::pin(async move {
            let url = format!("{}{}", client.config.base_address, route.get_route());
            let response = client
                .http_client
                .post(url)
                .send()
                .await
                .map_err(ClientError::ApiError)?;
            response.json().await.map_err(ClientError::ApiError)
        })
    }
}
