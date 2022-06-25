use std::fmt::Debug;

use iced::{
    pure::widget::{Button, Column, Container, Text},
    pure::{widget::Row, Element},
    Command, Length, Subscription,
};

use crate::gui::messages::Message;

#[allow(dead_code)]
pub enum Position {
    Top,
    Bottom,
}

pub trait Tab<'a, Message>
where
    Message: Debug + Clone,
{
    fn title(&self) -> String;
    fn subscription(&self) -> Subscription<Message>;
    fn view(&self) -> Element<'a, Message>;
    fn update(&mut self, event: Message) -> Command<Message>;
}

pub struct TabBar<'a, Message>
where
    Message: Debug + Clone,
{
    //needed later for styling selected tab
    current_tab: usize,
    tabs: Vec<Box<dyn Tab<'a, Message>>>,
    position: Position,
}

impl<'a> TabBar<'a, Message> {
    pub fn new(current_tab: usize, position: Position) -> Self {
        TabBar {
            current_tab,
            tabs: vec![],
            position,
        }
    }

    pub fn push(mut self, tab: impl Tab<'a, Message> + 'static) -> Self {
        self.tabs.push(Box::new(tab));
        self
    }

    pub fn subscription(&self) -> Subscription<Message> {
        self.tabs.get(self.current_tab).unwrap().subscription()
    }

    pub fn update(&mut self, event: Message) -> Command<Message> {
        match event {
            Message::TabSelected(i) => {
                self.current_tab = i;
                Command::none()
            }
            _ => self.tabs.get_mut(self.current_tab).unwrap().update(event),
        }
    }

    pub fn view(&self) -> iced::pure::Element<'a, Message> {
        let mut tab_row = Row::new()
            .align_items(iced::Alignment::Fill)
            .height(Length::Shrink)
            .width(Length::Fill);
        for i in 0..self.tabs.len() {
            tab_row = tab_row.push(
                Button::new(
                    Text::new(&self.tabs.get(i).unwrap().title())
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                        .vertical_alignment(iced::alignment::Vertical::Center),
                )
                .on_press(Message::TabSelected(i))
                .height(Length::Units(70))
                .width(Length::Fill),
            );
        }
        let content = Container::new(self.tabs.get(self.current_tab).unwrap().view())
            .width(Length::Fill)
            .height(Length::Fill);
        match self.position {
            Position::Bottom => Column::new().push(content).push(tab_row).into(),
            Position::Top => Column::new().push(tab_row).push(content).into(),
        }
    }
}
