use std::{process::Command, io::BufRead};

pub mod connection;
pub mod api;
pub mod models;

#[derive(Clone, Debug)]
pub struct Config { 
    base_address: String
 }

#[derive(Clone, Debug)]
pub struct Client {
    http_client: reqwest::Client,
    config: Config
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
        Self { http_client, config }
    }
}