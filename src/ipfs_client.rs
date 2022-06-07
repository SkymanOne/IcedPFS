use reqwest::{blocking};

pub enum Error {
    StandardError
}

pub fn request_files() -> Result<String, Error> {
    let client = blocking::Client::new();

    client.post("http://127.0.0.1:5001/api/v0/files/ls")
        .send()
        .map_err(|_x| Error::StandardError)
        .map(  |x| x.text().unwrap())
}