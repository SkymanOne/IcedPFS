use std::{fmt::Debug};

use iced::{
    pure::{button, row, text},
    pure::{
        Element,
    },
    Length,
};
use iced_lazy::pure::Component;

pub trait Tab<Message>
where
    Message: Debug + Clone,
{
    fn title(&self) -> String;
    fn view(&self) -> Element<'_, Message>;
}

#[derive(Debug, Clone)]
pub enum Event {
    TabChanged(usize),
}

pub struct TabBar<Message> {
    current_tab: usize,
    tabs: Vec<String>,
    on_change: Box<dyn Fn(usize) -> Message>,
}

impl<Message> TabBar<Message> {
    pub fn new(current_tab: usize, on_change: impl Fn(usize) -> Message + 'static) -> Self {
        TabBar {
            current_tab,
            tabs: vec![],
            on_change: Box::new(on_change),
        }
    }

    pub fn push(mut self, label: String) -> Self {
        self.tabs.push(label);
        self
    }
}

impl<Message, Renderer> Component<Message, Renderer> for TabBar<Message>
where
    Message: Debug + Clone,
    Renderer: iced_native::renderer::Renderer + iced_native::text::Renderer + 'static,
{
    type State = ();

    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::TabChanged(i) => Some((self.on_change)(i)),
        }
    }

    fn view(&self, _: &Self::State) -> iced_pure::Element<Self::Event, Renderer> {
        let mut tab_row = row()
            .align_items(iced::Alignment::Fill)
            .height(Length::Shrink)
            .width(Length::Fill);

        for i in 0..self.tabs.len() {
            tab_row = tab_row.push(
                button(
                    text(&self.tabs[i])
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .horizontal_alignment(iced::alignment::Horizontal::Center)
                        .vertical_alignment(iced::alignment::Vertical::Center),
                )
                .height(Length::Units(70))
                .width(Length::Fill),
            );
        }

        tab_row.into()
    }
}

impl<'a, Message, Renderer> From<TabBar<Message>> for iced_pure::Element<'a, Message, Renderer>
where
    Message: 'a + Debug + Clone,
    Renderer: iced_native::renderer::Renderer + iced_native::text::Renderer + 'static,
{
    fn from(numeric_input: TabBar<Message>) -> Self {
        iced_lazy::pure::component(numeric_input)
    }
}
