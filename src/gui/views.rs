pub mod home;
pub mod tab;
pub mod welcome;

#[derive(Debug, Clone, PartialEq)]
pub enum Views {
    WelcomeView,
    TabsView,
}
