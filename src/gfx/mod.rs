use ggez::context::Has;
use ggez::graphics::{Drawable, GraphicsContext, Rect, Text};
use ggez::mint::Point2;

pub mod scenes;

pub struct PositionedText {
    pub text: Text,
    pub position: Point2<f32>,
}

impl PositionedText {
    fn new(text: Text, position: Point2<f32>) -> Self {
        Self { text, position }
    }

    fn x(&self) -> f32 {
        self.position.x
    }

    fn y(&self) -> f32 {
        self.position.y
    }

    fn dimensions(&self, gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        self.text
            .dimensions(gfx)
            .map(|d| Rect::new(self.x() - (d.w / 2.0), self.y() - (d.h / 2.0), d.w, d.h))
    }
}

pub enum DrawableElementState {
    Hidden,
    Visible,
    Hovered,
    Clicked,
}