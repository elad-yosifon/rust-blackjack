use std::collections::VecDeque;

use crate::{at, take_stdin_key};
use crate::cards::card::{Card, CardSymbol};
use crate::gameplay::blackjack::{blackjack_card_value, UserAction};

pub enum HandState {
    Undefined,
    Finished,
    Bust,
    Blackjack,
}

impl HandState {
    pub fn from_value(value: i32) -> HandState {
        if value > 21 {
            return HandState::Bust;
        }
        if value == 21 {
            return HandState::Blackjack;
        }
        if value < 21 {
            return HandState::Undefined;
        }
        unreachable!()
    }
}

pub struct Hand {
    pub state: HandState,
    pub sum: i32,
    pub cards: VecDeque<Card>,
}

impl Hand {
    
    pub fn is_splitable(&self) -> bool {
        self.cards.len() == 2 && self.card_at(0).value == self.card_at(1).value
    }

    pub fn new() -> Self {
        Hand {
            cards: VecDeque::new(),
            sum: 0,
            state: HandState::Undefined,
        }
    }

    pub fn from_cards(cards: Vec<Card>) -> Self {
        Hand {
            cards: VecDeque::from(cards),
            sum: 0,
            state: HandState::Undefined,
        }
    }

    pub fn describe(&self) -> String {
        let mut strings = Vec::new();
        let mut fully_revealed = true;
        for card in self.cards.iter() {
            assert!(card.value != CardSymbol::Joker);
            if card.is_revealed() {
                strings.push(card.value.to_str().to_string());
            } else {
                fully_revealed = false;
                strings.push("X".to_string());
            }
        }

        let mut hand_str = strings.join(" + ");
        if fully_revealed {
            hand_str.push_str(format!(" = {}", self.sum).as_str());
        } else {
            hand_str.push_str(" = ?");
        }

        hand_str
    }

    pub fn card_at(&self, at: usize) -> &Card {
        at!(self.cards, at)
    }

    pub fn card_at_mut(&mut self, at: usize) -> &mut Card {
        at!(mut self.cards, at)
    }

    pub fn deal_card(&mut self, card: Card) {
        self.cards.push_back(card)
    }

    pub fn split(&mut self, card_0_1: Card, card_1_1: Card) -> Hand {
        let card_1_0 = self.cards.pop_back().unwrap();
        self.cards.push_back(card_0_1);
        Hand::from_cards(vec![card_1_0, card_1_1])
    }

    pub(crate) fn update_state(&mut self) {
        let mut sum = 0;
        let mut number_of_aces = 0;
        for card in self.cards.iter() {
            assert!(card.value != CardSymbol::Joker);
            sum += blackjack_card_value(&card.value);
            if card.value == CardSymbol::Ace {
                number_of_aces += 1;
            }
        }

        while number_of_aces > 0 {
            if sum <= 21 {
                break;
            }
            sum -= 10;
            number_of_aces -= 1;
        }

        self.sum = sum;

        if matches!(self.state, HandState::Undefined) {
            self.state = HandState::from_value(sum);
        }
    }

    pub(crate) fn prompt_user_action(
        &self,
        actor_name: &String,
        hand_idx: usize,
    ) -> UserAction {
        loop {
            if self.is_splitable() {
                let prompt = format!(
                    "{}:{} STAY/HIT/SPLIT? [s/h/x]:",
                    actor_name,
                    hand_idx + 1
                );
                match take_stdin_key!(prompt, 's', 'h', 'x') {
                    'h' => {
                        return UserAction::Hit;
                    }
                    's' => {
                        return UserAction::Stay;
                    }
                    'x' => {
                        return UserAction::Split;
                    }
                    _ => {
                        println!("Invalid command.");
                        continue;
                    }
                }
            } else {
                let prompt = format!(
                    "{}:{} STAY/HIT? [s/h]:",
                    actor_name,
                    hand_idx + 1
                );
                match take_stdin_key!(prompt, 's', 'h') {
                    'h' => {
                        return UserAction::Hit;
                    }
                    's' => {
                        return UserAction::Stay;
                    }
                    _ => {
                        println!("Invalid command.");
                        continue;
                    }
                }
            }
        }
    }
}
