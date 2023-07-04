use std::vec::Vec;

use crate::at;
use crate::gameplay::hand::Hand;

#[derive(Debug)]
pub enum ActorRole {
    Player,
    Dealer,
}

impl ActorRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActorRole::Player => "player",
            ActorRole::Dealer => "dealer",
        }
    }
}

pub struct Actor {
    pub role: ActorRole,
    pub hands: Vec<Hand>,
    pub actor_idx: usize,
}

impl Actor {
    pub fn hand_at(&self, at: usize) -> &Hand {
        at!(self.hands, at)
    }

    pub fn hand_at_mut(&mut self, at: usize) -> &mut Hand {
        at!(mut self.hands, at)
    }

    pub fn new(actor_idx: usize, hand: Hand) -> Self {
        Self {
            actor_idx,
            role: ActorRole::Player,
            hands: vec![hand],
        }
    }
}