use std::borrow::ToOwned;
use std::clone::Clone;
use std::collections::VecDeque;
use std::convert::{From, Into};
use std::iter::FromIterator;
use std::prelude::v1::derive;
use std::string::{String, ToString};

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy, Debug)]
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
    fn from_value(value: i32) -> CardSymbol {
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

    pub fn to_string(&self) -> String {
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
            CardSymbol::JOKER => "X"
        }.to_string()
    }
}

#[derive(Clone, Copy)]
pub enum Colors {
    RED,
    BLACK,
}

impl Colors {
    pub fn to_str(&self) -> &str {
        match &self {
            Colors::RED => "RED",
            Colors::BLACK => "BLACK"
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
    pub fn color(&self) -> Colors {
        match self {
            Suit::SPADE => Colors::BLACK,
            Suit::CLUB => Colors::BLACK,
            Suit::HEART => Colors::RED,
            Suit::DIAMOND => Colors::RED,
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            Suit::SPADE => "S",
            Suit::CLUB => "C",
            Suit::HEART => "H",
            Suit::DIAMOND => "D",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub suit: Suit,
    pub value: CardSymbol,
    pub revealed: bool,
}

impl Card {
    fn new(suit: Suit, value: CardSymbol) -> Self {
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

    pub fn describe(&self) {
        println!("Card: color={}, suit={}, value={}",
                 self.suit.color().to_str(),
                 self.suit.symbol(),
                 self.value.to_string()
        );
    }
}

pub struct Deck {
    pub cards: VecDeque<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards: VecDeque<Card> = VecDeque::new();
        let suits = [Suit::SPADE, Suit::HEART, Suit::CLUB, Suit::DIAMOND];
        for &suit in suits.iter() {
            for value in 1..14 { // skipping joker for now
                Deck::push_card_from_values(&mut cards, value, suit)
            }
        }
        Self { cards }
    }

    fn push_card_from_values(cards: &mut VecDeque<Card>, value: i32, suit: Suit) {
        let card: Card = Card::new(suit, CardSymbol::from_value(value));
        cards.push_back(card);
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        let mut slice = self.cards.make_contiguous();
        slice.shuffle(&mut rng);
        let mut new_deck: VecDeque<Card> = slice.to_vec().into();
        self.cards = new_deck;
    }
}
