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
    pub fn make_request<T, U>(&self, route: U, path: Option<PathBuf>) -> FutureResult<T>
    where
        U: ApiRoute<T> + Serialize + Send + 'static,
        T: DeserializeOwned + Default,
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
            let mut request = client.http_client.post(url);

            if let Some(path) = path {
                let file = tokio::fs::File::open(path)
                    .await
                    .map_err(|_| ClientError::FileError)?;
                let stream =
                    tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new());
                let body = reqwest::Body::wrap_stream(stream);

                let multipart = reqwest::multipart::Form::new()
                    .part("file", reqwest::multipart::Part::stream(body));
                request = request.multipart(multipart);
            }

            let response = request.send().await.map_err(ClientError::ApiError)?;
            let parsed = response.json().await.map_err(ClientError::ApiError);
            if parsed.is_ok() {
                parsed
            } else {
                Ok(T::default())
            }
        })
    }
}
