use iced::pure::widget::Text;
use iced::{Command, Subscription};

use crate::gui::Context;
use crate::gui::{messages::Message, widgets::tab_bar::Tab};

pub struct UploadTab {}

impl UploadTab {
    pub fn new() -> Self {
        UploadTab {}
    }
}

impl<'a> Tab<'a, Message> for UploadTab {
    fn title(&self) -> String {
        "Upload".to_string()
    }

    fn subscription(&self, _: &Context) -> Subscription<Message> {
        Subscription::none()
    }

    #[allow(unused_variables)]
    fn update(&mut self, event: Message, _: &Context) -> Command<Message> {
        Command::none()
    }

    fn view(&self, _: &Context) -> iced::pure::Element<Message> {
        Text::new("Upload tab").into()
    }
}
