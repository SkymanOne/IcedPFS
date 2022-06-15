use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use iced::pure::Application;
use iced::Command;

use crate::gui::messages::Message;
use crate::ipfs_client;
use crate::ipfs_client::api::ApiRequest;
use crate::ipfs_client::api::bandwidth::BandwidthStatsRequest;

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
    ipfs_client: IpfsRef,
    connection: ConnectionState,
}

#[derive(Debug, Clone)]
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
        let welcome_view = views::welcome::WelcomeView::new(client.clone());
        let home_view = views::home::HomeView::new(client.clone());
        (
            IcedPFS {
                view: Views::WelcomeView,
                welcome_view,
                home_view,
                ipfs_client: client,
                connection: ConnectionState::Disconnected,
            },
            connection_attempt(),
        )
    }

    fn title(&self) -> String {
        <&str>::into("IcedPFS")
    }

    fn update(&mut self, event: Self::Message) -> iced::Command<Self::Message> {
        match event {
            messages::Message::Route(route_action) => match route_action {
                Route::GoTo(view) => {
                    self.view = view;
                    iced::Command::none()
                }
                _ => iced::Command::none(),
            },
            messages::Message::ConnectionAttempt(success) => {
                if success {
                    self.connection = ConnectionState::Connected;
                    self.welcome_view.update(event);
                    Command::none()
                } else {
                    panic!("No ipfs installed!")
                }
            }
            Message::Tick => {
                //let statsRequest = BandwidthStatsRequest::new(self.ipfs_client.clone());
                let action = self.ipfs_client.bw_stats();
                Command::perform(
                    action,
                    |result| match result {
                        Ok(data) => Message::BwStatsReceived(data),
                        Err(_) => Message::Disconnected,
                    },
                )
            }
            Message::Disconnected => {
                println!("Client was disconnected! Connection attempt");
                connection_attempt()
            }
            Message::BwStatsReceived(_) => self.welcome_view.update(event),
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let tick_service = iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick);
        let mut services = vec![tick_service];
        match self.view {
            Views::WelcomeView => services.push(self.welcome_view.subscription()),
            Views::MainView => services.push(self.home_view.subscription())
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

fn connection_attempt() -> Command<Message> {
    iced::Command::perform(
        async move { ipfs_client::Client::join_network().is_ok() },
        Message::ConnectionAttempt,
    )
}
