use crate::{
    gui::{
        messages::{Message, Route},
        widgets::tab_bar::TabBar,
    },
    gui::{widgets::tab_bar::Tab, IpfsRef},
};
use iced::{
    pure::widget::{Button, Column, Container, Text, Row},
    Command, Length, Padding, Subscription,
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
        //file and folders can be potentially be represented as buttons with come content
        let btn = Button::new(
            Column::new()
                .push(Text::new("Go back"))
                .push(Text::new("Go back"))
                .align_items(iced::Alignment::Center),
        )
        .on_press(Message::Route(Route::GoTo(Views::WelcomeView)));
        let col = Column::new()
            .push(Text::new("Hello, world2!"))
            .push(btn)
            .push(TabBar::new(Message::TabSelected).push(Tab::new(
                String::from("Hello label"),
                Row::new().push(Text::new("Hello, world")).width(Length::Fill),
            )))
            .push(Text::new("Some text"))
            .spacing(5)
            .align_items(iced::Alignment::Center)
            .width(Length::Fill);
        Container::new(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
