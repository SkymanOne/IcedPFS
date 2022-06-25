use iced::{Command, Subscription, Text};

use crate::gui::{messages::Message, widgets::tab_bar::Tab};

pub struct SettingsTab {}

impl SettingsTab {
    pub fn new() -> Self {
        SettingsTab {}
    }
}

impl<'a> Tab<'a, Message> for SettingsTab {
    fn title(&self) -> String {
        "Settings".to_string()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        Subscription::none()
    }

    #[allow(unused_variables)]
    fn update(&mut self, event: Message) -> iced::Command<Message> {
        Command::none()
    }

    fn view(&self) -> iced::pure::Element<'a, Message> {
        Text::new("This is settings tab").into()
    }
}
