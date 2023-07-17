use ggez::glam::Vec2;
use ggez::graphics::{Canvas, DrawParam, Image};
use ggez::Context;

use crate::cards::card::{Card, CardSymbol, Suit};

pub struct CardsSprite {
    image: Image,
    anchor_x: u32,
    anchor_y: u32,
    anchor_w: u32,
    anchor_h: u32,
    step_x: u32,
    step_y: u32,
}

impl CardsSprite {
    pub fn new(ctx: &Context) -> Self {
        Self {
            image: Image::from_path(ctx, "/card_sprite.png").unwrap(),
            anchor_x: 16,
            anchor_y: 16,
            anchor_h: 270,
            anchor_w: 180,
            step_x: 195,
            step_y: 285,
        }
    }

    pub fn draw_card_at_point(&self, canvas: &mut Canvas, symbol:&CardSymbol, suit: &Suit, x: &f32, y: &f32) {
        let row = Self::row_from_suit(suit);
        let column = Self::column_from_symbol(symbol);
        let rect = self.image.uv_rect(
            self.anchor_x + (self.step_x * column),
            self.anchor_y + (self.step_y * row),
            self.anchor_w,
            self.anchor_h,
        );
        let draw_param = DrawParam::new().src(rect).dest(Vec2::new(*x, *y));
        canvas.draw(&self.image, draw_param)
    }

    fn column_from_symbol(symbol: &CardSymbol) -> u32 {
        match symbol {
            CardSymbol::Ace => 0,
            CardSymbol::Two => 1,
            CardSymbol::Three => 2,
            CardSymbol::Four => 3,
            CardSymbol::Five => 4,
            CardSymbol::Six => 5,
            CardSymbol::Seven => 6,
            CardSymbol::Eight => 7,
            CardSymbol::Nine => 8,
            CardSymbol::Ten => 9,
            CardSymbol::Jack => 10,
            CardSymbol::Queen => 11,
            CardSymbol::King => 12,
            CardSymbol::Joker => unreachable!(),
        }
    }

    fn row_from_suit(suit: &Suit) -> u32 {
        match suit {
            Suit::Heart => 0,
            Suit::Diamond => 1,
            Suit::Club => 2,
            Suit::Spade => 3,
        }
    }
}
