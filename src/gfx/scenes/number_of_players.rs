use std::ops::Not;
use ggez::event::MouseButton;
use ggez::graphics::{Canvas, Color, DrawParam, Drawable, PxScale, Text, TextFragment, TextLayout};
use ggez::mint::Point2;
use ggez::Context;
use ggez::input::mouse;
use ggez::input::mouse::CursorIcon;
use crate::gfx::scenes::Scene;
use crate::gfx::DrawableElementState::{Clicked, Hidden, Hovered, Visible};
use crate::gfx::{DrawableElementState, PositionedText};
use crate::MyGame;

pub struct NumberOfPlayersScene {
    elements: Vec<(PositionedText, DrawParam, DrawableElementState, bool)>,
}

impl NumberOfPlayersScene {
    pub fn new(game: &MyGame) -> Self {
        let (h, w) = game.window_size;

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
                new_positioned_text(txt, w / 2.0, 300., false),
                new_positioned_text(b1, 200., 500., true),
                new_positioned_text(b2, 400., 500., true),
                new_positioned_text(b3, 600., 500., true),
            ],
        }
    }
}

fn new_positioned_text(
    mut txt: Text,
    x: f32,
    y: f32,
    clickable: bool,
) -> (PositionedText, DrawParam, DrawableElementState, bool) {
    txt.set_layout(TextLayout::center());
    (
        PositionedText::new(txt, Point2::from([x, y])),
        DrawParam::default().dest(Point2::from([x, y])),
        Visible,
        clickable,
    )
}

impl Scene for NumberOfPlayersScene {
    fn update(&mut self, game: &mut MyGame, ctx: &mut Context) {

        mouse::set_cursor_type(ctx,CursorIcon::Default);

        let mouse_pos = ctx.mouse.position();

        if ctx.mouse.button_pressed(MouseButton::Left) {
            for (element, dp, s, clickable) in &self.elements {
                if let Some(r) = element.dimensions(&ctx.gfx) {
                    if r.contains(mouse_pos) {
                        //TODO: update state
                    }
                }
            }
        }

        for (i, (element, dp, element_state, clickable)) in self.elements.iter_mut().enumerate() {
            if clickable.not() {
                continue;
            }
            if let Some(r) = element.dimensions(&ctx.gfx) {
                if r.contains(mouse_pos) {
                    *element_state = Hovered;
                    mouse::set_cursor_type(ctx,CursorIcon::Hand);
                    (*dp).color = Color::YELLOW;
                } else {
                    *element_state = Visible;
                    (*dp).color = Color::WHITE;
                }
            }
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        self.elements
            .iter()
            .for_each(|(t, dp, element_state, clickable)| match element_state {
                Hidden => {}
                Clicked | Visible => {
                    canvas.draw(&t.text, *dp);
                }
                Hovered => {
                    canvas.draw(&t.text, *dp);
                }
            });
    }
}
