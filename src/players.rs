use std::ops::IndexMut;
use std::vec::Vec;

use crate::at;
use crate::cards::Card;

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
            return HandState::FINISHED;
        }
        unreachable!()
    }
}

#[derive(Debug)]
pub struct Hand {
    pub state: HandState,
    pub sum: i32,
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: vec![],
            sum: 0,
            state: HandState::UNDEFINED,
        }
    }

    #[allow(dead_code)]
    pub fn from_cards(cards: Vec<Card>) -> Self {
        Hand {
            cards,
            sum: 0,
            state: HandState::UNDEFINED,
        }
    }

    #[allow(dead_code)]
    pub fn describe(&self) {
        for &card in self.cards.iter() {
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
    pub fn deal_card(&mut self, card: Card) {
        self.deal_card_at_hand(0, card)
    }

    pub fn deal_card_at_hand(&mut self, hand_index: usize, card: Card) {
        self.hands.index_mut(hand_index).cards.push(card)
    }

    pub fn hand_at(&self, at: usize) -> &Hand {
        at!(self.hands, at)
    }

    pub fn hand_at_mut(&mut self, at: usize) -> &mut Hand {
        at!(mut self.hands, at)
    }
}

impl Player {
    pub fn new_dealer() -> Self {
        Self {
            role: PlayerRole::DEALER,
            hands: vec![Hand::new()],
        }
    }

    pub fn new() -> Self {
        Self {
            role: PlayerRole::PLAYER,
            hands: vec![Hand::new()],
        }
    }
}
