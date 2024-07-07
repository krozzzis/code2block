mod arrow;
mod diamond;
mod rectangle;
mod trapeze;

pub use arrow::*;
pub use diamond::*;
pub use rectangle::*;
pub use trapeze::*;

use iced::widget::canvas::Frame;

pub trait Figure {
    fn render(&self, frame: &mut Frame);
}
