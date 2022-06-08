use std::{process::Command, io::BufRead};
use reqwest::{blocking};

pub mod connection;
pub mod api;

#[derive(Clone, Debug)]
pub struct Config { 
    base_address: String
 }

#[derive(Clone, Debug)]
pub struct Client {
    http_client: blocking::Client,
    config: Config
}

#[derive(Debug)]
pub enum ClientError {
    NoIPFS,
    ErrorCreatingIPFS,
    ApiError(reqwest::Error)
}

impl Default for Client {
    fn default() -> Self {
        //create a client
        let config = Config { base_address: "http://127.0.0.1:5001".into() };
        let http_client = reqwest::blocking::Client::new();
        Self { http_client, config }
    }
}