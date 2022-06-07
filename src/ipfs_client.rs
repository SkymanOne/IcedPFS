use std::{process::Command, io::BufRead};
use iced::futures::stream;
use reqwest::{blocking, Request};

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

pub trait Operations {
    fn list_files(&self) -> Result<String, ClientError>;
}

impl Client {
    pub fn default() -> Result<Self, ClientError> {
        use std::process::Stdio;
        use std::io::BufReader;

        //running ipfs command and getting child pipe for output
        let pipe = Command::new("ipfs")
            .arg("daemon")
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .stdin(Stdio::null())
            .spawn()
            .map_err(|_| ClientError::NoIPFS)?;
        
        let mut std_reader = BufReader::new(pipe.stdout.unwrap());
        let mut err_reader = BufReader::new(pipe.stderr.unwrap());

        //set string to some initial value as we want to terminate loop only when it is empty
        let mut ready_string: String = "default".into();

        //toggle value to check whether we got expected output
        let mut daemon_started = false;

        //if ipfs node isn't already running then "ipfs daemon" would launch it
        while !ready_string.is_empty() && !daemon_started {
            ready_string = String::default();
            std_reader.read_line(&mut ready_string).map_err(|_| ClientError::ErrorCreatingIPFS)?;
            daemon_started = ready_string.contains("Daemon is ready");
        }

        //if ipfs node is already running when "ipfs daemon" would complain about the lock
        //we also reset the string for purpose stated above
        ready_string = "default".into();
        while !ready_string.is_empty() && !daemon_started {
            ready_string = String::default();
            err_reader.read_line(&mut ready_string).map_err(|_| ClientError::ErrorCreatingIPFS)?;
            daemon_started = ready_string.contains("someone else has the lock");
        }

        //if after two check the daemon hasn't started, probably it doesn't exist
        if !daemon_started {
            return Err(ClientError::ErrorCreatingIPFS);
        }
        
        //otherwise create a 
        let config = Config { base_address: "http://127.0.0.1:5001".into() };
        let http_client = reqwest::blocking::Client::new();
        Ok(Self { http_client, config })
    }
}


impl Operations for Client {

    /// Simple operation to list files in IPFS MFS
    fn list_files(&self) -> Result<String, ClientError> {
        let response = self.http_client.post(format!("{}/api/v0/files/ls", &self.config.base_address))
        .send()
        .map_err(ClientError::ApiError)?;
        response.text().map_err(ClientError::ApiError)
    }
}