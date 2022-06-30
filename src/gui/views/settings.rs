use iced::{Command, Subscription, Text};

use crate::gui::{messages::Message, widgets::tab_bar::Tab, Context};

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

    fn subscription(&self, _: &Context) -> Subscription<Message> {
        Subscription::none()
    }

    #[allow(unused_variables)]
    fn update(&mut self, event: Message, _: &Context) -> Command<Message> {
        Command::none()
    }

    fn view(&self, _: &Context) -> iced::pure::Element<Message> {
        Text::new("This is settings tab").into()
    }
}
