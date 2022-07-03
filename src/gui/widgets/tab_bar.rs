use std::fmt::Debug;

use iced::{
    pure::widget::{Button, Column, Container, Text},
    pure::{widget::Row, Element},
    Command, Length, Subscription,
};

use crate::gui::{messages::Message, Context};

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
    fn subscription(&self, ctx: &Context) -> Subscription<Message>;
    fn update(&mut self, event: Message, ctx: &Context) -> Command<Message>;
    fn view(&self, ctx: &Context) -> Element<Message>;
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

    pub fn subscription(&self, ctx: &Context) -> Subscription<Message> {
        self.tabs.get(self.current_tab).unwrap().subscription(ctx)
    }

    pub fn update(&mut self, event: Message, ctx: &Context) -> Command<Message> {
        match event {
            Message::TabSelected(i) => {
                self.current_tab = i;
                Command::none()
            }
            Message::Files(_) => {
                let mut cmds: Vec<Command<Message>> = vec![];
                self.tabs.iter_mut().for_each(|tab| {
                    cmds.push(tab.update(event.clone(), ctx));
                });
                Command::batch(cmds)
            }
            _ => self
                .tabs
                .get_mut(self.current_tab)
                .unwrap()
                .update(event, ctx),
        }
    }

    pub fn view(&self, ctx: &Context) -> Element<Message> {
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
        let content = Container::new(self.tabs.get(self.current_tab).unwrap().view(ctx))
            .width(Length::Fill)
            .height(Length::Fill);
        match self.position {
            Position::Bottom => Column::new().push(content).push(tab_row).into(),
            Position::Top => Column::new().push(tab_row).push(content).into(),
        }
    }
}
