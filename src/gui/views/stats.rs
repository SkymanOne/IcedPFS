use iced::{Command, Subscription, Text};

use crate::gui::{messages::Message, widgets::tab_bar::Tab, Context};

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

    fn subscription(&self, _: &Context) -> Subscription<Message> {
        Subscription::none()
    }

    #[allow(unused_variables)]
    fn update(&mut self, event: Message, _: &Context) -> Command<Message> {
        Command::none()
    }

    fn view(&self, _: &Context) -> iced::pure::Element<Message> {
        Text::new("This is stats tab. Custom graphs will appear here later").into()
    }
}
