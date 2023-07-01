use std::clone::Clone;
use std::prelude::v1::derive;

#[derive(PartialEq, Debug)]
pub enum CardSymbol {
    ACE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    JOKER,
}

impl CardSymbol {
    pub(crate) fn from_value(value: i32) -> CardSymbol {
        match value {
            1 => CardSymbol::ACE,
            2 => CardSymbol::TWO,
            3 => CardSymbol::THREE,
            4 => CardSymbol::FOUR,
            5 => CardSymbol::FIVE,
            6 => CardSymbol::SIX,
            7 => CardSymbol::SEVEN,
            8 => CardSymbol::EIGHT,
            9 => CardSymbol::NINE,
            10 => CardSymbol::TEN,
            11 => CardSymbol::JACK,
            12 => CardSymbol::QUEEN,
            13 => CardSymbol::KING,
            14 => CardSymbol::JOKER,
            _ => unreachable!("invalid card value {}!", value),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match &self {
            CardSymbol::ACE => "A",
            CardSymbol::TWO => "2",
            CardSymbol::THREE => "3",
            CardSymbol::FOUR => "4",
            CardSymbol::FIVE => "5",
            CardSymbol::SIX => "6",
            CardSymbol::SEVEN => "7",
            CardSymbol::EIGHT => "8",
            CardSymbol::NINE => "9",
            CardSymbol::TEN => "10",
            CardSymbol::JACK => "J",
            CardSymbol::QUEEN => "Q",
            CardSymbol::KING => "K",
            CardSymbol::JOKER => "X",
        }
    }
}

#[derive(Clone, Copy)]
pub enum Colors {
    RED,
    BLACK,
}

impl Colors {
    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match &self {
            Colors::RED => "RED",
            Colors::BLACK => "BLACK",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Suit {
    SPADE,
    CLUB,
    HEART,
    DIAMOND,
}

impl Suit {
    #[allow(dead_code)]
    pub fn color(&self) -> Colors {
        match self {
            Suit::SPADE => Colors::BLACK,
            Suit::CLUB => Colors::BLACK,
            Suit::HEART => Colors::RED,
            Suit::DIAMOND => Colors::RED,
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &'static str {
        match self {
            Suit::SPADE => "S",
            Suit::CLUB => "C",
            Suit::HEART => "H",
            Suit::DIAMOND => "D",
        }
    }
}

#[derive(Debug)]
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

    pub fn unreveal(&mut self) {
        self.revealed = false
    }

    pub fn is_revealed(&self) -> bool {
        self.revealed
    }

}
