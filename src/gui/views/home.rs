use crate::{
    gui::{messages::{Message, Route}, widgets::tab_bar::TabBar},
    gui::IpfsRef,
};
use iced::{
    pure::widget::{Button, Column, Container, Text},
    Command, Length, Subscription,
};

use super::Views;

//TODO: custom sidebar
pub struct HomeView {
    ipfs_client: IpfsRef,
}

impl HomeView {
    pub fn new(ipfs_client: IpfsRef) -> Self {
        HomeView { ipfs_client }
    }

    pub fn update(&mut self, event: Message) -> iced::Command<Message> {
        Command::none()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        Subscription::none()
    }

    pub fn view(&self) -> iced::pure::Element<Message> {
        let btn = Button::new("previous Screen")
            .on_press(Message::Route(Route::GoTo(Views::WelcomeView)));
        let col = Column::new()
            .push(Text::new("Hello, world2!"))
            .push(btn)
            .push(TabBar::new(Message::TabSelected))
            .spacing(5)
            .align_items(iced::Alignment::Center);
        Container::new(col)
            .padding(50)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
