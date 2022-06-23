use super::views::{self, home::MainMessage, tab::TabMessage};
use crate::ipfs_client::models::BandwidthStats;

#[derive(Debug, Clone)]
pub enum Message {
    Route(Route),
    ConnectionAttempt(bool),
    Disconnected,
    BwStatsReceived(BandwidthStats),
    Tick,
    TabSelected(usize),
    Tabs(TabMessage)
}

#[derive(Debug, Clone)]
pub enum Route {
    GoTo(views::Views),
    _Next,
    _Back,
}
