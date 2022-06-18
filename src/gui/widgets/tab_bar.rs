use iced::{Background, Color, Length, Point, Size};
use iced_native::layout::Node;
use iced_pure::{
    widget::{Row, Text},
    Element, Widget,
};

// TODO!: break down tab bar to two components: bar and the tab layout

pub struct Tab<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
{
    content: Element<'a, Message, Renderer>,
    label: String,
}

impl<'a, Message, Renderer> Tab<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
{
    pub fn new<E>(label: String, content: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        Self {
            label,
            content: content.into(),
        }
    }
}

pub struct TabBar<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
{
    tabs: Vec<Tab<'a, Message, Renderer>>,
    active_tab: usize,
    height: Length,
    width: Length,
    tab_height: f32,
}

impl<'a, Message, Renderer> TabBar<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
{
    pub fn new<F>(callback: F) -> Self
    where
        F: 'static + Fn(usize) -> Message,
    {
        TabBar {
            tabs: vec![],
            active_tab: 0,
            height: Length::Fill,
            width: Length::Fill,
            tab_height: 20.0,
        }
    }

    pub fn push(mut self, tab: Tab<'a, Message, Renderer>) -> Self {
        self.tabs.push(tab);
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for TabBar<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let size = limits
            .width(Length::Fill)
            .height(self.height)
            .resolve(Size::ZERO);
        let bar = iced_native::layout::Node::new(Size::new(size.width, self.tab_height));
        self.tabs
            .iter()
            .fold(Row::<Message, Renderer>::new(), |row, tab| {
                row.push(Text::new(&tab.label))
            })
            .width(self.width)
            .height(self.height)
            .layout(renderer, limits)
    }

    fn draw(
        &self,
        state: &iced_pure::widget::Tree,
        renderer: &mut Renderer,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) {
        renderer.fill_quad(
            iced_native::renderer::Quad {
                bounds: layout.bounds(),
                border_radius: 0.0,
                border_width: 1.0,
                border_color: iced_native::Color::BLACK,
            },
            iced_native::Color::BLACK,
        );
        let children = layout.children();

        for ((i, tab), layout) in self.tabs.iter().enumerate().zip(children) {
            renderer.fill_quad(
                iced_native::renderer::Quad {
                    bounds: layout.bounds(),
                    border_radius: 0.0,
                    border_width: 1.0,
                    border_color: Color::BLACK,
                },
                Color::WHITE,
            );
            // renderer.fill_text(iced_native::text::Text {
            //     content: &tab.label,
            //     font: iced_native::Font::Default,
            //     size: 11.0,
            //     bounds: layout.children().next().expect("error").bounds(),
            //     color: Color::BLACK,
            //     horizontal_alignment: iced::alignment::Horizontal::Center,
            //     vertical_alignment: iced::alignment::Vertical::Center
            // })
        }
    }
}

impl<'a, Message, Renderer> From<TabBar<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
    Message: 'a + Clone,
{
    fn from(tab_bar: TabBar<'a, Message, Renderer>) -> Self {
        Element::new(tab_bar)
    }
}
