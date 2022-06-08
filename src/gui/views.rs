use iced::{pure::Element, Command, Subscription};

pub mod welcome;
pub mod home;


#[derive(Debug, Clone, PartialEq)]
pub enum Views {
    WelcomeView,
    MainView
}