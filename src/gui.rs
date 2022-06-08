use std::rc::Rc;

use iced::pure::{Sandbox, widget::{Text}};

use crate::ipfs_client;

use self::views::{BaseView, welcome_view::{WelcomeMessage, WelcomeView, self}, Views};

pub mod views;

pub type IpfsRef = Rc<ipfs_client::Client>;

#[derive(Debug)]
pub enum GeneralMessage {
    WelcomeMessage(WelcomeMessage),
    MainMessage(())
}

pub struct IcedPFS { 
    view: Views,
    welcome_view: WelcomeView,
    ipfs_client: IpfsRef,
}

impl Sandbox for IcedPFS{
    type Message = GeneralMessage;
    fn new() -> Self {
        let client = Rc::new(ipfs_client::Client::default());
        let welcome_view = views::welcome_view::WelcomeView::new(Rc::clone(&client));
        IcedPFS { view: Views::WelcomeView, 
            ipfs_client: client,
            welcome_view }
    }

    fn title(&self) -> String {
        "IcedPFS".into()
    }

    fn update(&mut self, event: Self::Message) {
        match event {
            GeneralMessage::WelcomeMessage(welcome_msg) => { self.welcome_view.update(welcome_msg); },
            GeneralMessage::MainMessage(_) => todo!(),
        }
    }

    fn view(&self) -> iced::pure::Element<Self::Message> {
        match self.view {
            Views::WelcomeView => {
                self.welcome_view.view().map(GeneralMessage::WelcomeMessage)
            },
            Views::MainView => Text::new("Hello, World2").into()
        }
    }
}
