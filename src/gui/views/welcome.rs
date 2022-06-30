use crate::{
    gui::messages::{Message, Route},
    gui::{views::Views, Context},
    gui::IpfsRef,
    ipfs_client::models::BandwidthStats,
};
use iced::{
    pure::widget::{Button, Column, Container, Text},
    Length,
};

#[derive(Debug, Clone)]
pub struct WelcomeView {
    _ipfs_client: IpfsRef,
    progress_messages: Vec<String>,
    stats: BandwidthStats,
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
            stats: BandwidthStats::default(),
        }
    }

    pub fn update(&mut self, event: Message) -> iced::Command<Message> {
        match event {
            Message::BwStatsReceived(stats) => {
                self.stats = stats;
            }
            Message::ConnectionAttempt(success) => {
                self.progress_messages.pop();
                if success {
                    self.progress_messages
                        .push("Connected to IPFS network!".to_string());
                } else {
                    self.progress_messages
                        .push("You don't seem to have IPFS node installed!".to_string());
                }
            }
            _ => {}
        }

        iced::Command::none()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::none()
    }

    pub fn view(&self, ctx: &Context) -> iced::pure::Element<Message> {
        let msg = Message::Route(Route::GoTo(Views::TabsView));
        let btn = Button::new("Proceed").on_press(msg);

        let stats = Text::new(self.stats.to_string());

        let progress_col = self.progress_messages.iter().fold(
            Column::new()
                .spacing(10)
                .align_items(iced::Alignment::Center),
            |column, el| column.push(Text::new(el)),
        );

        let mut main_col = Column::new()
            .align_items(iced::Alignment::Center)
            .spacing(130)
            .push(Text::new("Welcome to IcedPFS"))
            .push(progress_col)
            .push(stats);

        if ctx.is_connected() {
            main_col = main_col.push(btn);
        }

        Container::new(main_col)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
