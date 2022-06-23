pub mod tab;
pub mod welcome;
pub mod home;

#[derive(Debug, Clone, PartialEq)]
pub enum Views {
    WelcomeView,
    TabsView,
}
