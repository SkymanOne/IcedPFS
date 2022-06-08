use iced::{Sandbox, Settings};

mod gui;
mod ipfs_client;

use gui::IcedPFS;
use crate::ipfs_client::api::Operations;


fn main() -> iced::Result {
    let client = ipfs_client::Client::default();
    if ipfs_client::Client::join_network().is_ok() {
        let body = client.list_files();
        println!("{:?}", body);
    }
    println!("Starting GUI");
    println!("Leaving network");
    match client.leave_network() {
       Ok(_) => println!("Client disconnected successfully"),
       _ => print!("Client was disconnected before. All good!")
    }
    IcedPFS::run(Settings::default())
}
