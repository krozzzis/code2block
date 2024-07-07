use iced::{
    alignment::{Horizontal, Vertical},
    widget::{
        canvas::{
            fill::Rule, path::Builder, Fill, Frame, LineCap, LineDash, LineJoin, Stroke, Style,
            Text,
        },
        text::{LineHeight, Shaping},
    },
    Color, Font, Pixels, Point,
};

use crate::{figures::Figure, vec::Vec2};

pub struct Rectange {
    pub pos: Vec2,
    pub w: f32,
    pub h: f32,
    pub stroke_color: Color,
    pub fill_color: Color,
    pub text: String,
}

impl Figure for Rectange {
    fn render(&self, frame: &mut Frame) {
        let mut builder = Builder::new();
        builder.move_to(Point::new(
            self.pos.x - self.w / 2.0,
            self.pos.y + self.h / 2.0,
        ));
        builder.line_to(Point::new(
            self.pos.x - self.w / 2.0,
            self.pos.y - self.h / 2.0,
        ));
        builder.line_to(Point::new(
            self.pos.x + self.w / 2.0,
            self.pos.y - self.h / 2.0,
        ));
        builder.line_to(Point::new(
            self.pos.x + self.w / 2.0,
            self.pos.y + self.h / 2.0,
        ));
        builder.close();
        let path = builder.build();
        frame.fill(
            &path,
            Fill {
                style: Style::Solid(self.fill_color),
                rule: Rule::NonZero,
            },
        );
        frame.stroke(
            &path,
            Stroke {
                style: Style::Solid(self.stroke_color),
                width: 2.0,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                line_dash: LineDash::default(),
            },
        );
        let txt = Text {
            content: self.text.clone(),
            position: Point::new(self.pos.x, self.pos.y),
            color: Color::BLACK,
            size: Pixels(self.w.min(self.h) / 2.0),
            line_height: LineHeight::Relative(0.5),
            font: Font::DEFAULT,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            shaping: Shaping::Basic,
        };
        frame.fill_text(txt);
    }
}
