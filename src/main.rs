use iced::{Sandbox, Settings};

mod gui;
mod ipfs_client;

use gui::IcedPFS;


fn main() -> iced::Result {
    println!("Starting GUI");
    let body = ipfs_client::request_files();
    if let Ok(b) = body {
        println!("{}", b);
    }
    IcedPFS::run(Settings::default())
}
