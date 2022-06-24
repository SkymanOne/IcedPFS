use crate::{
    gui::IpfsRef,
    gui::{
        messages::Message,
        widgets::tab_bar::{TabBar, Position},
    },
};

use iced::{
    pure::widget::Text,
    Command, Subscription,
};

use super::home::{HomeMessage, HomeTab};


#[derive(Debug, Clone)]
pub enum TabMessage {
    Home(HomeMessage)
}

pub struct TabsView {
    main_view: HomeTab,
    pub current_tab: usize,
}

impl TabsView {
    pub fn new(ipfs_client: IpfsRef) -> (Self, Command<Message>) {
        let current_tab = 0;
        let main = HomeTab::new(ipfs_client);
        let view = TabsView {
            current_tab,
            main_view: main.0
        };
        (view, Command::batch([main.1]))
    }

    pub fn update(&mut self, event: TabMessage) -> iced::Command<Message> {
        match event {
            TabMessage::Home(msg) => self.main_view.update(msg)
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        Subscription::none()
    }

    pub fn view(&self) -> iced::pure::Element<Message> {
        //file and folders can be potentially be represented as buttons with come content
        TabBar::new(self.current_tab, Position::Bottom)
            .push("Home".to_string(), self.main_view.view().map(|msg| Message::Tabs(TabMessage::Home(msg))))
            .push("Upload".to_string(), Text::new("Upload").into())
            .push("Network Stats".to_string(), Text::new("Network stats").into())
            .push("Settings".to_string(), Text::new("Settings").into())
            .view()
    }
}
