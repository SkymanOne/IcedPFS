use iced::{pure::Element, Command, Subscription};

pub mod welcome_view;

pub trait BaseView<T> {
    fn update(&mut self, event: T) -> Command<T>;
    fn subscription(&self) -> Subscription<T>;
    fn view(&self) -> Element<T>;
}

pub enum Views {
    WelcomeView,
    MainView
}