use std::fmt::Debug;

use iced::{
    pure::{
        widget::{Button, Column, Row, Text},
        Element,
    },
    Command, Length,
};

//TODO: implement lazy component

pub struct Tab<Message> where Message: Debug + Clone {
    content: Element<'static, Message>,
    title: String,
}

pub struct TabBarNavigation<Message> where Message: Debug + Clone {
    current_tab: usize,
    tabs: Vec<Tab<Message>>,
}

enum Event {
    TabChanged(usize),
}

impl<Message> TabBarNavigation<Message> where Message: Debug + Clone + 'static {
    pub fn new(current_tab: usize) -> Self {
        TabBarNavigation {
            current_tab,
            tabs: vec![],
        }
    }

    pub fn view(&self) -> Element<'static, Message> {
        let i = self.current_tab ;
        let mut column = Column::new().push(self.tabs[i].content);
        let mut tab_row: Row<Message> = Row::new()
            .align_items(iced::Alignment::Fill)
            .height(Length::Shrink)
            .width(Length::Fill);

        for el in self.tabs {
            tab_row.push(
                Button::new(
                    Text::new(el.title.clone())
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                        .vertical_alignment(iced::alignment::Vertical::Center)
                )
                .height(Length::Units(70))
                .width(Length::Fill),
            );
        }
        column.width(Length::Fill).height(Length::Fill).into()
    }
}
