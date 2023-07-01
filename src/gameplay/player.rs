use std::vec::Vec;

use crate::at;
use crate::gameplay::hand::Hand;


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

    pub fn new(hand: Hand) -> Self {
        Self {
            role: PlayerRole::PLAYER,
            hands: vec![hand],
        }
    }
}
