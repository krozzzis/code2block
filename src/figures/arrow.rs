use iced::{
    widget::canvas::{
        fill::Rule, path::Builder, Fill, Frame, LineCap, LineDash, LineJoin, Stroke, Style,
    },
    Color, Point,
};

use crate::{figures::Figure, vec::Vec2};

pub struct Arrow {
    pub points: Vec<Vec2>,
    pub color: Color,
}

impl Figure for Arrow {
    fn render(&self, frame: &mut Frame) {
        let mut builder = Builder::new();
        if let Some(first) = self.points.first() {
            builder.move_to(Point::new(first.x, first.y));
        }
        for point in self.points.iter().skip(1) {
            builder.line_to(Point::new(point.x, point.y));
        }
        let path = builder.build();
        frame.stroke(
            &path,
            Stroke {
                style: Style::Solid(self.color),
                width: 2.0,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                line_dash: LineDash::default(),
            },
        );
        /*
        // Draw circles on segment joints
        for point in self.points.iter().skip(1).take(self.points.len() - 2) {
            let path = Path::circle(Point::new(point.x, point.y), 3.0);
            frame.fill(&path, Fill {
                style: Style::Solid(self.color),
                rule: Rule::NonZero,
            });
        }
        */
        if let Some(last) = self.points.last() {
            if let Some(first) = self.points.iter().nth_back(1) {
                let dir = *last - *first;
                let dir = dir / dir.lenght();
                let size = 6.0;
                let p1 = Vec2::new(-dir.y, dir.x) * size / 2.0 + *last - dir * size;
                let p2 = Vec2::new(dir.y, -dir.x) * size / 2.0 + *last - dir * size;
                let mut builder = Builder::new();
                builder.move_to(Point::new(last.x, last.y));
                builder.line_to(Point::new(p1.x, p1.y));
                builder.line_to(Point::new(p2.x, p2.y));
                builder.close();
                frame.fill(
                    &builder.build(),
                    Fill {
                        style: Style::Solid(self.color),
                        rule: Rule::NonZero,
                    },
                );
            }
        }
    }
}
