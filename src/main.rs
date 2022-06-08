use iced::{pure::Sandbox, Settings};

mod gui;
mod ipfs_client;

use gui::IcedPFS;
use crate::ipfs_client::api::Operations;


fn main() -> iced::Result {
    println!("Starting GUI");
    IcedPFS::run(Settings::default())
}
