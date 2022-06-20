use iced_lazy::pure::Component;
use iced_pure::{Element, widget::Text};

//TODO: implement lazy component

pub struct TabBarNavigation { }

impl<Message, Renderer> Component<Message, Renderer> for TabBarNavigation {
    type State = ();

    type Event = ();

    fn update(
        &mut self,
        state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        todo!()
    }

    fn view(&self, state: &Self::State) -> Element<Self::Event, Renderer> {
        todo!();
    }
}