use std::hash::Hash;

use crate::{
    gui::messages::{Message, Route},
    gui::views::Views,
    gui::IpfsRef,
    ipfs_client::{self, api::Operations, Client},
};
use iced::{
    pure::{
        widget::{Button, Column, Container, Text},
    },
    Length,
};

#[derive(Debug, Clone)]
pub struct WelcomeView {
    ipfs_client: IpfsRef,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct State {
    progress_messages: Vec<String>,
}

impl WelcomeView {
    pub fn new(ipfs_client: IpfsRef) -> Self {
        WelcomeView { ipfs_client }
    }

    pub fn update(&mut self, event: Message) -> iced::Command<Message> {
        //TODO: handle connection updates
        iced::Command::none()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::none()
    }

    pub fn view(&self) -> iced::pure::Element<Message> {
        let msg = Message::Route(Route::GoTo(Views::MainView));
        let btn = Button::new("next Screen").on_press(msg);
        let col = Column::new()
            .push(Text::new("Hello, world!"))
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
