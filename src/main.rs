use iced::{pure::Application, Settings};

mod gui;
mod ipfs_client;

use gui::IcedPFS;


fn main() -> iced::Result {
    println!("Starting GUI");
    IcedPFS::run(Settings::default())
}
