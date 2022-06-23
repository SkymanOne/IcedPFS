use crate::{gui::messages::Message, gui::IpfsRef};

use iced::pure::widget::Text;
use iced::{Command, Subscription};

#[derive(Debug, Clone, Copy)]
pub enum MainMessage {}

pub struct HomeTab {
    ipfs_client: IpfsRef,
}

impl HomeTab {
    pub fn new(ipfs_client: IpfsRef) -> Self {
        HomeTab { ipfs_client }
    }

    pub fn update(&mut self, event: MainMessage) -> iced::Command<Message> {
        Command::none()
    }

    pub fn view(&self) -> iced::pure::Element<Message> {
        Text::new("This is the main tab").into()
    }
}
