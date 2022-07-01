use crate::{
    gui::IpfsRef,
    gui::{
        messages::Message,
        widgets::tab_bar::{Position, TabBar}, Context,
    },
};

use iced::Command;

use super::{home::HomeTab, settings::SettingsTab, stats::StatsTab, upload::UploadTab};

pub struct TabsView<'a> {
    tab_bar: TabBar<'a, Message>,
}

impl<'a> TabsView<'a> {
    pub fn new(ipfs_client: IpfsRef) -> (Self, Command<Message>) {
        let main = HomeTab::new(ipfs_client.clone());
        let upload = UploadTab::new(ipfs_client);
        let stats = StatsTab::new();
        let settings = SettingsTab::new();
        let tab_bar = TabBar::new(0, Position::Bottom)
            .push(main.0)
            .push(upload)
            .push(stats)
            .push(settings);
        let view = TabsView { tab_bar };
        (view, Command::batch([main.1]))
    }

    pub fn update(&mut self, event: Message, ctx: &Context) -> Command<Message> {
        self.tab_bar.update(event, ctx)
    }

    pub fn subscription(&self, ctx: &Context) -> iced::Subscription<Message> {
        self.tab_bar.subscription(ctx)
    }

    pub fn view(&self, ctx: &Context) -> iced::pure::Element<Message> {
        self.tab_bar.view(ctx)
    }
}
