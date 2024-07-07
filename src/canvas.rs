use iced::mouse;
use iced::{
    widget::canvas::{Cache, Frame, Geometry, Program},
    Renderer, Theme,
};

use crate::figures::Figure;

pub struct Viewer<'a> {
    pub render_list: &'a Vec<Box<dyn Figure>>,
    pub cache: &'a Cache,
}

impl<'a> Program<()> for Viewer<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let content = self
            .cache
            .draw(renderer, bounds.size(), |frame: &mut Frame| {
                for figure in self.render_list {
                    figure.render(frame);
                }
            });
        vec![content]
    }
}
