use iced::pure::{widget::Text, Sandbox};

use super::views;

#[derive(Debug, Clone)]
pub enum Message {
    Route(Route),
    ConnectedToIPFS
}


#[derive(Debug, Clone)]
pub enum Route {
    GoTo(views::Views),
    Next,
    Back
}
