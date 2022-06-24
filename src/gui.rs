use std::time::Duration;

use iced::pure::Application;
use iced::{Command, Subscription};

use crate::gui::messages::Message;
use crate::ipfs_client;
use crate::ipfs_client::api::stats::BandwidthStatsRequest;

use self::messages::Route;
use self::views::tab::TabsView;
use self::views::welcome::WelcomeView;
use self::views::Views;

pub mod messages;
pub mod views;
mod widgets;

pub type IpfsRef = ipfs_client::Client;

pub struct IcedPFS {
    view: Views,
    welcome_view: WelcomeView,
    tabs_view: TabsView,
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
        let client = ipfs_client::Client::default();
        let welcome_view = views::welcome::WelcomeView::new(client.clone());
        let tabs_view = views::tab::TabsView::new(client.clone());
        (
            IcedPFS {
                view: Views::WelcomeView,
                welcome_view,
                tabs_view: tabs_view.0,
                ipfs_client: client,
                connection: ConnectionState::Disconnected,
            },
            Command::batch([connection_attempt(), tabs_view.1]),
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
                // example using idiomatic requests
                // let request = BandwidthStatsRequest::new()
                //     .with_peer("12D3KooWER6HhMejRszMhUuCdcCyk9S2gWbph96NaazPHxLkfzPF");
                let request = BandwidthStatsRequest::new();
                let action = self.ipfs_client.make_request(request);
                Command::perform(action, |result| match result {
                    Ok(data) => Message::BwStatsReceived(data),
                    Err(_) => Message::Disconnected,
                })
            }
            Message::Disconnected => {
                println!("Client was disconnected! Connection attempt");
                connection_attempt()
            }
            Message::BwStatsReceived(_) => self.welcome_view.update(event),
            Message::TabSelected(i) => {
                self.tabs_view.current_tab = i;
                Command::none()
            }
            Message::Tabs(msg) => self.tabs_view.update(msg),
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let tick_service = iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick);
        let mut services = vec![tick_service];
        match self.view {
            Views::WelcomeView => services.push(self.welcome_view.subscription()),
            Views::TabsView => services.push(self.tabs_view.subscription()),
        }

        Subscription::batch(services.into_iter())
    }

    fn view(&self) -> iced::pure::Element<Self::Message> {
        match self.view {
            Views::WelcomeView => self.welcome_view.view(),
            Views::TabsView => self.tabs_view.view(),
        }
    }
}

fn connection_attempt() -> Command<Message> {
    iced::Command::perform(
        async move { ipfs_client::Client::join_network().is_ok() },
        Message::ConnectionAttempt,
    )
}
