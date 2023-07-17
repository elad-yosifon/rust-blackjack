use ggez::graphics::Text;
use ggez::graphics::TextFragment;
use ggez::graphics::{Canvas, DrawParam};
use ggez::graphics::{Color, PxScale};
use ggez::input::mouse;
use ggez::input::mouse::CursorIcon;
use ggez::mint::Point2;
use ggez::Context;

use crate::gfx::elements::drawable_element::DrawableElement;
use crate::gfx::scenes::Scene;
use crate::GameContext;

pub struct EndOfGameScene {
    elements: Vec<DrawableElement>,
}

impl EndOfGameScene {
    pub fn new(ctx: &Context) -> Self {
        let (_, w) = ctx.gfx.size();

        let mut txt = Text::new(TextFragment::new("Until next time..."));
        txt.set_scale(PxScale::from(60.0));

        Self {
            elements: vec![DrawableElement::new_text(
                txt,
                Point2::from([w / 2.0, 300.]),
            )],
        }
    }
}

impl Scene for EndOfGameScene {
    fn update(&mut self, game_ctx: &mut GameContext, ctx: &mut Context) {
        mouse::set_cursor_type(ctx, CursorIcon::Default);
    }

    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) {
        self.elements
            .iter()
            .filter(|de| de.is_visible())
            .for_each(|element| {
                if element.is_hovered() {
                    element.draw(ctx, canvas, DrawParam::new().color(Color::YELLOW));
                } else {
                    element.draw(ctx, canvas, DrawParam::new().color(Color::WHITE));
                }
            });
    }
}
