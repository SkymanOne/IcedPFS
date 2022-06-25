pub mod home;
pub mod settings;
pub mod stats;
pub mod tab;
pub mod upload;
pub mod welcome;

#[derive(Debug, Clone, PartialEq)]
pub enum Views {
    WelcomeView,
    TabsView,
}
