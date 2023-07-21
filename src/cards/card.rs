use std::clone::Clone;
use std::prelude::v1::derive;

#[derive(PartialEq, Copy, Clone)]
pub enum CardSymbol {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Joker,
}

impl CardSymbol {
    pub(crate) fn from_value(value: i32) -> CardSymbol {
        match value {
            1 => CardSymbol::Ace,
            2 => CardSymbol::Two,
            3 => CardSymbol::Three,
            4 => CardSymbol::Four,
            5 => CardSymbol::Five,
            6 => CardSymbol::Six,
            7 => CardSymbol::Seven,
            8 => CardSymbol::Eight,
            9 => CardSymbol::Nine,
            10 => CardSymbol::Ten,
            11 => CardSymbol::Jack,
            12 => CardSymbol::Queen,
            13 => CardSymbol::King,
            14 => CardSymbol::Joker,
            _ => unreachable!("invalid card value {}!", value),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match &self {
            CardSymbol::Ace => "A",
            CardSymbol::Two => "2",
            CardSymbol::Three => "3",
            CardSymbol::Four => "4",
            CardSymbol::Five => "5",
            CardSymbol::Six => "6",
            CardSymbol::Seven => "7",
            CardSymbol::Eight => "8",
            CardSymbol::Nine => "9",
            CardSymbol::Ten => "10",
            CardSymbol::Jack => "J",
            CardSymbol::Queen => "Q",
            CardSymbol::King => "K",
            CardSymbol::Joker => "X",
        }
    }
}

pub enum Colors {
    Red,
    Black,
}

impl Colors {
    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match &self {
            Colors::Red => "Red",
            Colors::Black => "Black",
        }
    }
}

#[derive(Copy, Clone)]
pub enum Suit {
    Spade,
    Club,
    Heart,
    Diamond,
}

impl Suit {
    #[allow(dead_code)]
    pub fn color(&self) -> Colors {
        match self {
            Suit::Spade => Colors::Black,
            Suit::Club => Colors::Black,
            Suit::Heart => Colors::Red,
            Suit::Diamond => Colors::Red,
        }
    }

    #[allow(dead_code)]
    pub fn to_str(self) -> &'static str {
        match self {
            Suit::Spade => "S",
            Suit::Club => "C",
            Suit::Heart => "H",
            Suit::Diamond => "D",
        }
    }
}

pub struct Card {
    pub suit: Suit,
    pub value: CardSymbol,
    pub revealed: bool,
}

impl Card {
    pub(crate) fn new(suit: Suit, value: CardSymbol) -> Self {
        Card {
            suit,
            value,
            revealed: true,
        }
    }

    pub fn reveal(&mut self) {
        self.revealed = true
    }

    pub fn hide(&mut self) {
        self.revealed = false
    }

    pub fn is_revealed(&self) -> bool {
        self.revealed
    }
}
