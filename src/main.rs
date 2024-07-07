mod canvas;
mod figures;
mod vec;

use canvas::*;

use iced::{
    executor,
    widget::{canvas::Cache, Canvas},
    Application, Color, Command, Length, Theme,
};

use crate::{
    figures::{Arrow, Diamond, Figure, Rectange, Trapeze},
    vec::Vec2,
};

struct Code2block {
    render_list: Vec<Box<dyn Figure>>,
    canvas_cache: Cache,
}

impl Application for Code2block {
    type Executor = executor::Default;

    type Message = ();

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                canvas_cache: Cache::default(),
                render_list: vec![
                    Box::new(Trapeze {
                        pos: Vec2::new(150.0, 50.0),
                        w: 120.0,
                        h: 40.0,
                        strafe: 20.0,
                        fill_color: Color::WHITE,
                        stroke_color: Color::BLACK,
                        text: String::from("начало"),
                    }),
                    Box::new(Arrow {
                        points: vec![Vec2::new(150.0, 70.0), Vec2::new(150.0, 90.0)],
                        color: Color::BLACK,
                    }),
                    Box::new(Rectange {
                        pos: Vec2::new(150.0, 110.0),
                        w: 90.0,
                        h: 40.0,
                        fill_color: Color::WHITE,
                        stroke_color: Color::BLACK,
                        text: String::from("x := 10"),
                    }),
                    Box::new(Arrow {
                        points: vec![Vec2::new(150.0, 130.0), Vec2::new(150.0, 150.0)],
                        color: Color::BLACK,
                    }),
                    Box::new(Rectange {
                        pos: Vec2::new(150.0, 170.0),
                        w: 90.0,
                        h: 40.0,
                        fill_color: Color::new(0.95, 0.85, 0.85, 1.0),
                        stroke_color: Color::new(0.9, 0.2, 0.2, 1.0),
                        text: String::from("y := 20"),
                    }),
                    Box::new(Arrow {
                        points: vec![Vec2::new(150.0, 190.0), Vec2::new(150.0, 210.0)],
                        color: Color::BLACK,
                    }),
                    Box::new(Diamond {
                        pos: Vec2::new(150.0, 230.0),
                        w: 120.0,
                        h: 40.0,
                        fill_color: Color::new(0.95, 0.85, 0.85, 1.0),
                        stroke_color: Color::new(0.9, 0.2, 0.2, 1.0),
                        text: String::from("x > 10"),
                    }),
                    Box::new(Arrow {
                        points: vec![
                            Vec2::new(150.0 + 60.0, 230.0),
                            Vec2::new(150.0 + 70.0, 230.0),
                            Vec2::new(150.0 + 70.0, 260.0),
                            Vec2::new(150.0, 260.0),
                            Vec2::new(150.0, 270.0),
                        ],
                        color: Color::BLACK,
                    }),
                    Box::new(Rectange {
                        pos: Vec2::new(150.0, 290.0),
                        w: 90.0,
                        h: 40.0,
                        fill_color: Color::new(0.95, 0.85, 0.85, 1.0),
                        stroke_color: Color::new(0.9, 0.2, 0.2, 1.0),
                        text: String::from("y := 20"),
                    }),
                    Box::new(Arrow {
                        points: vec![Vec2::new(150.0, 310.0), Vec2::new(150.0, 340.0)],
                        color: Color::BLACK,
                    }),
                    Box::new(Arrow {
                        points: vec![
                            Vec2::new(150.0 - 60.0, 230.0),
                            Vec2::new(150.0 - 70.0, 230.0),
                            Vec2::new(150.0 - 70.0, 320.0),
                            Vec2::new(150.0, 320.0),
                            Vec2::new(150.0, 340.0),
                        ],
                        color: Color::BLACK,
                    }),
                    Box::new(Trapeze {
                        pos: Vec2::new(150.0, 360.0),
                        w: 120.0,
                        h: 40.0,
                        strafe: 20.0,
                        fill_color: Color::WHITE,
                        stroke_color: Color::BLACK,
                        text: String::from("конец"),
                    }),
                ],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Code2block")
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let canvas = Canvas::new(Viewer {
            cache: &self.canvas_cache,
            render_list: &self.render_list,
        })
        .width(Length::Fill)
        .height(Length::Fill);
        canvas.into()
    }
}

fn main() -> iced::Result {
    Code2block::run(iced::Settings {
        antialiasing: true,
        ..Default::default()
    })
}
