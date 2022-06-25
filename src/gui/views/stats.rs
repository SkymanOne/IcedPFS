use iced::{Command, Subscription, Text};

use crate::gui::{messages::Message, widgets::tab_bar::Tab};

pub struct StatsTab {}

impl StatsTab {
    pub fn new() -> Self {
        StatsTab {}
    }
}

impl<'a> Tab<'a, Message> for StatsTab {
    fn title(&self) -> String {
        "Stats".to_string()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        Subscription::none()
    }

    #[allow(unused_variables)]
    fn update(&mut self, event: Message) -> iced::Command<Message> {
        Command::none()
    }

    fn view(&self) -> iced::pure::Element<'a, Message> {
        Text::new("This is stats tab. Custom graphs will appear here later").into()
    }
}
