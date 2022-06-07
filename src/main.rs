use iced::{Sandbox, Settings};

mod gui;
mod ipfs_client;

use gui::IcedPFS;

use crate::ipfs_client::Operations;


fn main() -> iced::Result {
    let created_client = ipfs_client::Client::default();
    if let Ok(client) = created_client {
        let body = client.list_files();
        println!("{:?}", body);
    }
    
    println!("Starting GUI");
    IcedPFS::run(Settings::default())
}
