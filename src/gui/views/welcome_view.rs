use crate::{
    gui::IpfsRef,
    ipfs_client::{self, api::Operations, Client},
};
use iced::{
    pure::{
        widget::{Container, Text},
        Sandbox,
    },
    Length,
};

use super::BaseView;

#[derive(Debug, Clone)]
pub struct WelcomeView {
    ipfs_client: IpfsRef,
}

#[derive(Debug, Default, Clone)]
struct State {
    progress_messages: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum WelcomeMessage {
    Loading,
    Loaded(State),
}

impl WelcomeView {
    pub fn new(ipfs_client: IpfsRef) -> Self {
        WelcomeView { ipfs_client }
    }
}
impl<WelcomeMessage: 'static> BaseView<WelcomeMessage> for WelcomeView {
    fn update(&mut self, event: WelcomeMessage) -> iced::Command<WelcomeMessage> {
        todo!()
    }

    fn subscription(&self) -> iced::Subscription<WelcomeMessage> {
        todo!()
    }

    fn view(&self) -> iced::pure::Element<WelcomeMessage> {
        if ipfs_client::Client::join_network().is_ok() {
            let body = self.ipfs_client.list_files();
            println!("{:?}", body);
        }
        println!("Leaving network");
        match self.ipfs_client.leave_network() {
            Ok(_) => println!("Client disconnected successfully"),
            _ => print!("Client was disconnected before. All good!"),
        }
        Container::new(Text::new("Hello, World1"))
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
