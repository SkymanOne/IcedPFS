use crate::{
    gui::messages::{Message, Route},
    gui::views::Views,
    gui::IpfsRef,
};
use iced::{
    pure::widget::{Button, Column, Container, Text},
    Length,
};

#[derive(Debug, Clone)]
pub struct WelcomeView {
    _ipfs_client: IpfsRef,
    progress_messages: Vec<String>,
}

impl WelcomeView {
    pub fn new(ipfs_client: IpfsRef) -> Self {
        let progress_messages = vec![
            "Started app!".to_string(),
            "Connecting to IPFS network...".to_string(),
        ];
        WelcomeView {
            _ipfs_client: ipfs_client,
            progress_messages,
        }
    }

    pub fn update(&mut self, event: Message) -> iced::Command<Message> {
        if let Message::ConnectedToIPFS = event {
            self.progress_messages.pop();
            self.progress_messages
                .push("Connected to IPFS successfully!".into());
        }
        iced::Command::none()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::none()
    }

    pub fn view(&self) -> iced::pure::Element<Message> {
        let msg = Message::Route(Route::GoTo(Views::MainView));
        let btn = Button::new("next Screen").on_press(msg);
        let progress_col = self
            .progress_messages
            .iter()
            .fold(Column::new().spacing(10).align_items(iced::Alignment::Center), 
            |column, el| column.push(Text::new(el)));

        let main_col = Column::new()
        .align_items(iced::Alignment::Center)
        .spacing(130)
        .push(Text::new("Welcome to IcedPFS"))
        .push(progress_col)
        .push(btn);

        Container::new(main_col)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
