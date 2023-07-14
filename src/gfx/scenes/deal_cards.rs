use ggez::Context;
use ggez::graphics::{Canvas, DrawParam};
use ggez::graphics::{Color, PxScale};
use ggez::graphics::Text;
use ggez::graphics::TextFragment;
use ggez::input::mouse;
use ggez::input::mouse::CursorIcon;
use ggez::mint::Point2;

use crate::GameContext;
use crate::gfx::elements::drawable_element::{DrawableElement, DrawableElementState};
use crate::gfx::elements::utils::iter_visible;
use crate::gfx::scenes::Scene;

pub struct DealCardsScene {
    elements: Vec<DrawableElement>,
    number_of_players: Vec<usize>,
}

impl DealCardsScene {
    pub fn new(game_ctx: &GameContext) -> Self {
        let (_, w) = game_ctx.window_size;

        let mut txt = Text::new(TextFragment::new("Number of players:"));
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
                DrawableElement::new_text_button(b1, Point2::from([200., 500.])).hidden(),
                DrawableElement::new_text_button(b2, Point2::from([400., 500.])).hidden(),
                DrawableElement::new_text_button(b3, Point2::from([600., 500.])).hidden(),
            ],
            number_of_players: vec![0, 1, 2, 3],
        }
    }
}

impl Scene for DealCardsScene {
    fn update(&mut self, game_ctx: &mut GameContext, ctx: &mut Context) {
        mouse::set_cursor_type(ctx, CursorIcon::Default);

        let n = game_ctx.game.number_of_players;
        let idx = self.number_of_players.get(n).unwrap();
        self.elements.get_mut(*idx).unwrap().show();

        // let mouse_pos = ctx.mouse.position();
        //
        // //TODO: be nice and support "click-zone" states
        // if ctx.mouse.button_pressed(MouseButton::Left) {
        //     for (i, element) in self.elements.iter().enumerate() {
        //         if element.possible_states.has(DrawableElementState::PRESSED) {
        //             if let Some(r) = element.dimensions(&ctx.gfx) {
        //                 if r.contains(mouse_pos) {
        //
        //                     /* TODO: implement */
        //
        //                     // not returning on purpose
        //                 }
        //             }
        //         }
        //     }
        // }

        // handle_hovers(game_ctx, ctx, &mut self.elements);
    }

    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) {
        iter_visible(&self.elements).for_each(|de| {
            if de.element_state.has(DrawableElementState::HOVERED) {
                de.draw(ctx, canvas, DrawParam::new().color(Color::YELLOW));
            } else {
                de.draw(ctx, canvas, DrawParam::new().color(Color::WHITE));
            }
        });
    }
}
