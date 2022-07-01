use std::{io::BufRead, path::PathBuf, pin::Pin, process::Command, sync::Arc};

use futures::Future;
use serde::{de::DeserializeOwned, Serialize};

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
    FileError,
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
        U: ApiRoute<T> + Serialize + Send + 'static,
        T: DeserializeOwned,
    {
        let client = self.clone();
        Box::pin(async move {
            let result = serde_urlencoded::to_string(&route);

            let params = match result {
                Ok(val) => val,
                Err(_) => String::new(),
            };

            let url = format!(
                "{}{}?{}",
                client.config.base_address,
                route.get_route(),
                params
            );
            let response = client
                .http_client
                .post(url)
                .send()
                .await
                .map_err(ClientError::ApiError)?;
            response.json().await.map_err(ClientError::ApiError)
        })
    }

    pub fn make_request_with_files<T, U>(&self, route: U, path: PathBuf) -> FutureResult<T>
    where
        U: ApiRoute<T> + Serialize + Send + 'static,
        T: DeserializeOwned,
    {
        let client = self.clone();
        Box::pin(async move {
            let result = serde_urlencoded::to_string(&route);

            let params = match result {
                Ok(val) => val,
                Err(_) => String::new(),
            };

            let url = format!(
                "{}{}?{}",
                client.config.base_address,
                route.get_route(),
                params
            );

            let file = tokio::fs::File::open(path)
                .await
                .map_err(|_| ClientError::FileError)?;
            let stream =
                tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new());
            let body = reqwest::Body::wrap_stream(stream);

            let multipart = reqwest::multipart::Form::new()
                .part("file", reqwest::multipart::Part::stream(body));

            let response = client
                .http_client
                .post(url)
                .multipart(multipart)
                .send()
                .await
                .map_err(ClientError::ApiError)?;
            response.json().await.map_err(ClientError::ApiError)
        })
    }
}
