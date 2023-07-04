use std::vec::Vec;

use crate::at;
use crate::gameplay::hand::Hand;

#[derive(Debug)]
pub enum ActorRole {
    Player,
    Dealer,
}

pub struct Actor {
    pub role: ActorRole,
    pub hands: Vec<Hand>,
    pub name: String,
}

impl Actor {
    pub fn hand_at(&self, at: usize) -> &Hand {
        at!(self.hands, at)
    }

    pub fn hand_at_mut(&mut self, at: usize) -> &mut Hand {
        at!(mut self.hands, at)
    }

    pub fn new(name: String, hand: Hand) -> Self {
        Self {
            name,
            role: ActorRole::Player,
            hands: vec![hand],
        }
    }
}
