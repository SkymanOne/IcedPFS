use std::fmt::Debug;

use iced::{
    pure::widget::{Button, Column, Text},
    pure::{widget::Row, Element},
    Length,
};

use crate::gui::messages::Message;

pub enum Position {
    Top,
    Bottom,
}

pub trait Tab<Message>
where
    Message: Debug + Clone,
{
    fn title(&self) -> String;
    fn view(&self) -> Element<'_, Message>;
}

pub struct TabBar<'a, Message> {
    //needed later for styling selected tab
    current_tab: usize,
    tabs: Vec<String>,
    tab_views: Vec<Element<'a, Message>>,
    position: Position,
}

impl<'a> TabBar<'a, Message> {
    pub fn new(current_tab: usize, position: Position) -> Self {
        TabBar {
            current_tab,
            tabs: vec![],
            tab_views: vec![],
            position,
        }
    }

    pub fn push(mut self, label: String, content: Element<'a, Message>) -> Self {
        self.tabs.push(label);
        self.tab_views.push(content);
        self
    }

    pub fn view(self) -> iced::pure::Element<'a, Message> {
        let mut tab_row = Row::new()
            .align_items(iced::Alignment::Fill)
            .height(Length::Shrink)
            .width(Length::Fill);
        for i in 0..self.tabs.len() {
            tab_row = tab_row.push(
                Button::new(
                    Text::new(&self.tabs[i])
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
        match self.position {
            Position::Bottom => Column::new()
                .push(self.tab_views.into_iter().nth(self.current_tab).unwrap())
                .push(tab_row)
                .into(),
            Position::Top => Column::new()
                .push(tab_row)
                .push(self.tab_views.into_iter().nth(self.current_tab).unwrap())
                .into(),
        }
    }
}
