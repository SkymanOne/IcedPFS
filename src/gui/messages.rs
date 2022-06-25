use crate::ipfs_client::models::{BandwidthStats, FileEntry, FilesList};

use super::views;

#[derive(Debug, Clone)]
pub enum Message {
    Route(Route),
    ConnectionAttempt(bool),
    Disconnected,
    BwStatsReceived(BandwidthStats),
    Tick,
    TabSelected(usize),
    Files(Files),
}

#[derive(Debug, Clone)]
pub enum Files {
    ListReceived(FilesList),
    FileClicked(FileEntry),
    FailedToFetch,
}

#[derive(Debug, Clone)]
pub enum Route {
    GoTo(views::Views),
}
