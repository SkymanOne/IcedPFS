use crate::{
    gui::messages::{Message, Route},
    gui::IpfsRef,
};
use iced::{
    pure::widget::{Button, Column, Container, Row, Text},
    Command, Length, Padding, Rule, Subscription,
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

        let back_btn = Button::new(Text::new("Go back"))
            .on_press(Message::Route(Route::GoTo(Views::WelcomeView)));

        Column::new()
            .push(
                Container::new(
                    Column::new()
                        .push(Text::new("Content number 1"))
                        .push(back_btn),
                )
                .height(Length::Fill)
                .width(Length::Fill)
                .center_x()
                .center_y(),
            )
            .push(
                Row::new()
                    .push(
                        Button::new(
                            Text::new("Tab1")
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .horizontal_alignment(iced::alignment::Horizontal::Center)
                                .vertical_alignment(iced::alignment::Vertical::Center),
                        )
                        .height(Length::Units(70))
                        .width(Length::Fill),
                    )
                    .push(
                        Button::new(
                            Text::new("Tab2")
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .horizontal_alignment(iced::alignment::Horizontal::Center)
                                .vertical_alignment(iced::alignment::Vertical::Center),
                        )
                        .height(Length::Units(70))
                        .width(Length::Fill),
                    )
                    .push(
                        Button::new(
                            Text::new("Tab3")
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .horizontal_alignment(iced::alignment::Horizontal::Center)
                                .vertical_alignment(iced::alignment::Vertical::Center),
                        )
                        .height(Length::Units(70))
                        .width(Length::Fill),
                    )
                    .align_items(iced::Alignment::Fill)
                    .height(Length::Shrink)
                    .width(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}