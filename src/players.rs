use std::convert::Into;
use std::ops::{Index, IndexMut};
use std::vec::Vec;

use crate::cards::Card;

pub struct PlayerState {}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand { cards: vec![] }
    }

    pub fn describe(&self) {
        for &card in self.cards.iter() {
            card.describe()
        }
    }
}

#[derive(Debug)]
pub enum PlayerRole {
    PLAYER,
    DEALER,
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
