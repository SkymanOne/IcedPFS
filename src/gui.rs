use iced::{Sandbox, Container, Text, Length};

pub struct IcedPFS { }

impl Sandbox for IcedPFS {
    type Message = ();

    fn new() -> Self {
        IcedPFS { }
    }

    fn title(&self) -> String {
        "IcedPFS".into()
    }

    fn update(&mut self, _message: Self::Message) {
        
    }

    fn view(&mut self) -> iced::Element<Self::Message> {
        Container::new(Text::new("Hello, World"))
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
