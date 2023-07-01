use std::collections::VecDeque;
use std::vec::Vec;

use crate::at;
use crate::cards::card::Card;

#[derive(Debug)]
pub enum HandState {
    UNDEFINED,
    FINISHED,
    BUST,
    BLACKJACK,
}

impl HandState {
    pub fn from_value(value: i32) -> HandState {
        if value > 21 {
            return HandState::BUST;
        }
        if value == 21 {
            return HandState::BLACKJACK;
        }
        if value < 21 {
            return HandState::UNDEFINED;
        }
        unreachable!()
    }
}

#[derive(Debug)]
pub struct Hand {
    pub state: HandState,
    pub sum: i32,
    pub cards: VecDeque<Card>,
}

impl Hand {
    pub fn splitable(&self) -> bool {
        if self.cards.len() == 2 && self.card_at(0).value == self.card_at(1).value {
            return true;
        }
        false
    }
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: VecDeque::new(),
            sum: 0,
            state: HandState::UNDEFINED,
        }
    }

    #[allow(dead_code)]
    pub fn from_cards(cards: Vec<Card>) -> Self {
        Hand {
            cards: VecDeque::from(cards),
            sum: 0,
            state: HandState::UNDEFINED,
        }
    }

    #[allow(dead_code)]
    pub fn describe(&self) {
        for card in self.cards.iter() {
            card.describe()
        }
    }

    #[allow(dead_code)]
    pub fn card_at(&self, at: usize) -> &Card {
        at!(self.cards, at)
    }

    #[allow(dead_code)]
    pub fn card_at_mut(&mut self, at: usize) -> &mut Card {
        at!(mut self.cards, at)
    }

    pub fn deal_card(&mut self, card: Card) {
        self.cards.push_back(card)
    }

    pub fn split(&mut self, card_0_1: Card, card_1_1: Card) -> Hand {
        let card_1_0 = self.cards.pop_back().unwrap();
        self.cards.push_back(card_0_1);
        let new_hand = Hand::from_cards(vec![card_1_0, card_1_1]);
        new_hand
    }
}

#[derive(Debug)]
pub enum PlayerRole {
    PLAYER,
    DEALER,
}

impl PlayerRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            PlayerRole::PLAYER => "player",
            PlayerRole::DEALER => "dealer",
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub role: PlayerRole,
    pub hands: Vec<Hand>,
}

impl Player {
    pub fn hand_at(&self, at: usize) -> &Hand {
        at!(self.hands, at)
    }

    pub fn hand_at_mut(&mut self, at: usize) -> &mut Hand {
        at!(mut self.hands, at)
    }
}

impl Player {
    pub fn new(hand: Hand) -> Self {
        Self {
            role: PlayerRole::PLAYER,
            hands: vec![hand],
        }
    }
}
