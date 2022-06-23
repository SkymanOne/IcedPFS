use crate::{
    gui::IpfsRef,
    gui::{
        messages::Message,
        widgets::tab_bar::{TabBar, Position},
    },
};
use iced::{
    pure::widget::{Button, Column, Container, Row, Text},
    Command, Length, Padding, Rule, Subscription,
};

//TODO: custom sidebar
pub struct HomeView {
    ipfs_client: IpfsRef,
    pub current_tab: usize,
}

impl HomeView {
    pub fn new(ipfs_client: IpfsRef) -> Self {
        let current_tab = 0;
        HomeView {
            ipfs_client,
            current_tab,

        }
    }

    pub fn update(&mut self, event: Message) -> iced::Command<Message> {
        Command::none()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        Subscription::none()
    }

    pub fn view(&self) -> iced::pure::Element<Message> {
        //file and folders can be potentially be represented as buttons with come content

        let content = Text::new("Hello, World")
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .horizontal_alignment(iced::alignment::Horizontal::Center)
                    .vertical_alignment(iced::alignment::Vertical::Center);

        let tabbar = TabBar::new(self.current_tab, Position::Bottom)
            .push("Tab 1".to_string(), content.into())
            .push("Tab 2".to_string(), Text::new("2").width(Length::Fill).height(Length::Fill).into())
            .push("Tab 3".to_string(), Text::new("3").width(Length::Fill).height(Length::Fill).into());
        tabbar.view()
    }
}
