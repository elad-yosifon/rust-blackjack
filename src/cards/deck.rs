use std::collections::VecDeque;

use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::cards::card::{Card, CardSymbol, Suit};

pub struct Deck {
    pub cards: VecDeque<Card>,
}

impl Deck {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            cards: VecDeque::from(Self::fresh_cards_vec()),
        }
    }

    pub fn new_shuffled() -> Self {
        let mut cards = Self::fresh_cards_vec();
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);
        Self {
            cards: VecDeque::from(cards),
        }
    }

    fn fresh_cards_vec() -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();
        let suits = vec![Suit::Spade, Suit::Heart, Suit::Club, Suit::Diamond];
        for &suit in suits.iter() {
            for value in 1..14 {
                // skipping joker for now
                cards.push(Card::new(suit, CardSymbol::from_value(value)));
            }
        }
        cards
    }

    pub fn draw_card(&mut self) -> Card {
        self.cards.pop_front().unwrap()
    }
}
