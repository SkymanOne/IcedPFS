use iced::{Background, Length, Size};
use iced_native::Renderer;
use iced_pure::{Element, Widget};

pub struct TabBar<'a, Message, Renderer> {
    tabs: Vec<Element<'a, Message, Renderer>>,
    height: Length,
    width: Length,
}

impl<'a, Message, Renderer> TabBar<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced_native::Renderer,
{
    pub fn new<F>(callback: F) -> Self
    where
        F: 'static + Fn(usize) -> Message,
    {
        TabBar {
            tabs: vec![],
            height: Length::Shrink,
            width: Length::Fill,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for TabBar<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: iced_native::Renderer,
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
        let width = limits.width(Length::Fill).resolve(Size::ZERO);
        iced_native::layout::Node::new(Size::new(width.width, 30.0))
    }

    fn draw(
        &self,
        state: &iced_pure::widget::Tree,
        renderer: &mut Renderer,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) {
        renderer.fill_quad(
            iced_native::renderer::Quad {
                bounds: layout.bounds(),
                border_radius: 10.0,
                border_width: 2.0,
                border_color: iced_native::Color::BLACK,
            },
            iced_native::Color::BLACK,
        );
    }
}

impl<'a, Message, Renderer> From<TabBar<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::Renderer,
    Message: 'a + Clone
{
    fn from(tab_bar: TabBar<'a, Message, Renderer>) -> Self {
        Element::new(tab_bar)
    }
}
