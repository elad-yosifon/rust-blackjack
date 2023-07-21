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

pub struct NumberOfPlayersScene {
    elements: Vec<DrawableElement>,
    number_of_players: Vec<usize>,
}

impl NumberOfPlayersScene {
    pub fn new(ctx: &Context) -> Self {
        let (_, w) = ctx.gfx.size();

        let mut txt = Text::new(TextFragment::new("Please choose number of players:"));
        txt.set_scale(PxScale::from(40.0));

        let mut b1 = Text::new(TextFragment::new("1"));
        b1.set_scale(PxScale::from(60.0));

        let mut b2 = Text::new(TextFragment::new("2"));
        b2.set_scale(PxScale::from(60.0));

        let mut b3 = Text::new(TextFragment::new("3"));
        b3.set_scale(PxScale::from(60.0));

        Self {
            elements: vec![
                DrawableElement::new_text(txt, Point2::from([w / 2.0, 300.])),
                DrawableElement::new_text_button(b1, Point2::from([200., 500.])),
                DrawableElement::new_text_button(b2, Point2::from([400., 500.])),
                DrawableElement::new_text_button(b3, Point2::from([600., 500.])),
            ],
            number_of_players: vec![0, 1, 2, 3],
        }
    }
}

impl Scene for NumberOfPlayersScene {
    fn update(&mut self, game_ctx: &mut GameContext, ctx: &mut Context) {
        mouse::set_cursor_type(ctx, CursorIcon::Default);

        for (i, element) in self.elements.iter_mut().enumerate() {
            if element.check_clicked(&ctx) {
                let number_of_players = self.number_of_players[i];
                game_ctx.start_game(number_of_players);
                game_ctx.game.start_new_round();
                game_ctx.current_scene = SceneType::PlayRound;
                break;
            }
        }

        self.elements.iter_mut().for_each(|element| {
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
