use std::sync::Arc;

use iced::pure::Application;
use iced_native::subscription;

use crate::gui::messages::Message;
use crate::ipfs_client;
use crate::ipfs_client::api::Operations;

use self::messages::Route;
use self::views::home::HomeView;
use self::views::welcome::WelcomeView;
use self::views::Views;

pub mod messages;
pub mod views;

pub type IpfsRef = Arc<ipfs_client::Client>;

pub struct IcedPFS {
    view: Views,
    welcome_view: WelcomeView,
    home_view: HomeView,
}

#[derive(Debug)]
enum ConnectionState {
    Disconnected,
    Connected,
}

impl Application for IcedPFS {
    type Executor = iced::executor::Default;
    type Message = messages::Message;
    type Flags = ();


    fn new(_: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let client = Arc::new(ipfs_client::Client::default());
        let welcome_view = views::welcome::WelcomeView::new(Arc::clone(&client));
        let home_view = views::home::HomeView::new(Arc::clone(&client));
        (IcedPFS {
            view: Views::WelcomeView,
            welcome_view,
            home_view,
        }, iced::Command::none())
    }

    fn title(&self) -> String {
        <&str>::into("IcedPFS")
    }

    fn update(&mut self, event: Self::Message) -> iced::Command<Self::Message> {
        match event {
            messages::Message::Route(route_action) => match route_action {
                Route::GoTo(view) => {
                self.view = view;
                iced::Command::none()},
                _ => iced::Command::none(),
            },
            messages::Message::ConnectedToIPFS => {
                self.welcome_view.update(event)
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let ipfs_service = connect_to_ipfs();
        let mut services = vec![ipfs_service];
        if self.view == Views::WelcomeView {
            services.push(self.welcome_view.subscription());
        }
        iced_native::Subscription::batch(services.into_iter())
     }

    fn view(&self) -> iced::pure::Element<Self::Message> {
        match self.view {
            Views::WelcomeView => self.welcome_view.view(),
            Views::MainView => self.home_view.view(),
        }
    }
}

fn connect_to_ipfs() -> iced::Subscription<Message> {
    struct Connector;
    subscription::unfold(
        std::any::TypeId::of::<Connector>(),
        ConnectionState::Disconnected,
        |state| async move {
            match state {
                ConnectionState::Disconnected => {
                    if ipfs_client::Client::join_network().is_ok() {
                        let ipfs = ipfs_client::Client::default();
                        let body = ipfs.list_files();
                        println!("{:?}", body);
                    }
                    (Some(Message::ConnectedToIPFS), ConnectionState::Connected)
                }
                //TODO: regularly check for connection status
                ConnectionState::Connected => (None, ConnectionState::Connected)
            }
        },
    )
}