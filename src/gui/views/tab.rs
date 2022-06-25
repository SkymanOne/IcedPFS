use crate::{
    gui::IpfsRef,
    gui::{
        messages::Message,
        widgets::tab_bar::{Position, TabBar},
    },
};

use iced::Command;

use super::{home::HomeTab, upload::UploadTab};

pub struct TabsView<'a> {
    tab_bar: TabBar<'a, Message>,
}

impl<'a> TabsView<'a> {
    pub fn new(ipfs_client: IpfsRef) -> (Self, Command<Message>) {
        let main = HomeTab::new(ipfs_client);
        let upload = UploadTab::new();
        let tab_bar =         TabBar::new(0, Position::Bottom)
            .push(main.0)
            .push(upload);
        let view = TabsView {
            tab_bar
        };
        (view, Command::batch([main.1]))
    }

    pub fn update(&mut self, event: Message) -> iced::Command<Message> {
        self.tab_bar.update(event)
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        self.tab_bar.subscription()
    }

    pub fn view(&self) -> iced::pure::Element<Message> {
        self.tab_bar.view()
    }
}
