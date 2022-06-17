pub mod home;
pub mod welcome;

#[derive(Debug, Clone, PartialEq)]
pub enum Views {
    WelcomeView,
    MainView,
}
