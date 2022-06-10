use super::{Client, ClientError};

pub trait Operations {
    fn list_files(&self) -> Result<String, ClientError>;
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