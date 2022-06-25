pub mod home;
pub mod tab;
pub mod welcome;
pub mod upload;

#[derive(Debug, Clone, PartialEq)]
pub enum Views {
    WelcomeView,
    TabsView,
}
