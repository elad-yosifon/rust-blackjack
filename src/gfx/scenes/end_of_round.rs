use ggez::Context;
use ggez::graphics::{Canvas, DrawParam};
use ggez::graphics::{Color, PxScale};
use ggez::graphics::Text;
use ggez::graphics::TextFragment;
use ggez::input::mouse;
use ggez::input::mouse::CursorIcon;
use ggez::mint::Point2;

use crate::GameContext;
use crate::gfx::elements::drawable_element::DrawableElement;
use crate::gfx::elements::utils::handle_hover_gfx;
use crate::gfx::scenes::{Scene, SceneType};

pub struct EndOfRoundScene {
    elements: Vec<DrawableElement>,
    values: Vec<bool>,
}

impl EndOfRoundScene {
    pub fn new(ctx: &Context) -> Self {
        let (_, w) = ctx.gfx.size();

        let mut txt = Text::new(TextFragment::new("Should we play another round?"));
        txt.set_scale(PxScale::from(40.0));

        let mut y = Text::new(TextFragment::new("YES"));
        y.set_scale(PxScale::from(60.0));

        let mut n = Text::new(TextFragment::new("NO"));
        n.set_scale(PxScale::from(60.0));

        Self {
            elements: vec![
                DrawableElement::new_text(txt, Point2::from([w / 2.0, 300.])),
                DrawableElement::new_text_button(y, Point2::from([200., 500.])),
                DrawableElement::new_text_button(n, Point2::from([400., 500.])),
            ],
            values: vec![false, true, false],
        }
    }
}

impl Scene for EndOfRoundScene {
    fn update(&mut self, game_ctx: &mut GameContext, ctx: &mut Context) {
        mouse::set_cursor_type(ctx, CursorIcon::Default);

        for (i, element) in self.elements.iter_mut().enumerate() {
            if element.check_clicked(ctx) {
                if self.values[i] {
                    game_ctx.current_scene = SceneType::PlayRound;
                } else {
                    game_ctx.current_scene = SceneType::EndOfGame;
                }
                break;
            }
        }

        let elements = &mut self.elements;
        elements.iter_mut().for_each(|element| {
            handle_hover_gfx(ctx, element);
        })
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
