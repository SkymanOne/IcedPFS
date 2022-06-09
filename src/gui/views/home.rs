use crate::{
    gui::messages::{Message, Route},
    gui::IpfsRef
};
use iced::{
    pure::{
        widget::{Container, Text, Button, Column},
    },
    Length,
};

use super::Views;

#[derive(Debug, Clone)]
pub struct HomeView {
    ipfs_client: IpfsRef,
}

impl HomeView {
    pub fn new(ipfs_client: IpfsRef) -> Self {
        HomeView { ipfs_client }
    }

    pub fn update(&mut self, event: Message) -> iced::Command<Message> {
        todo!()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        todo!()
    }

    pub fn view(&self) -> iced::pure::Element<Message> {
        let btn = Button::new("previous Screen").on_press(Message::Route(Route::GoTo(Views::WelcomeView)));
        let col = Column::new()
            .push(Text::new("Hello, world2!"))
            .push(btn)
            .spacing(5);
        Container::new(col)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
