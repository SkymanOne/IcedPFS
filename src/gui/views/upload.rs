use iced::{Subscription, Command};
use iced::pure::widget::Text;

use crate::gui::{widgets::tab_bar::Tab, messages::Message};

pub struct UploadTab {

}

impl UploadTab {
    pub fn new() -> Self {
        UploadTab {  }
    }
}

impl<'a> Tab<'a, Message> for UploadTab {
    fn title(&self) -> String {
        "Upload".to_string()
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        Subscription::none()
    }

    #[allow(unused_variables)]
    fn update(&mut self, event: Message) -> iced::Command<Message> {
        Command::none()
    }

    fn view(&self) -> iced::pure::Element<'a, Message> {
        Text::new("Upload tab").into()
    }

}