use std::collections::VecDeque;

use crate::cards::card::{Card, CardSymbol};
use crate::gameplay::blackjack::{blackjack_card_value, UserAction};
use crate::{at, read_stdin_str};

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
    fn splitable(&self) -> bool {
        if self.cards.len() == 2 && self.card_at(0).value == self.card_at(1).value {
            return true;
        }
        false
    }

    pub fn new() -> Self {
        Hand {
            cards: VecDeque::new(),
            sum: 0,
            state: HandState::UNDEFINED,
        }
    }

    pub fn from_cards(cards: Vec<Card>) -> Self {
        Hand {
            cards: VecDeque::from(cards),
            sum: 0,
            state: HandState::UNDEFINED,
        }
    }

    pub fn describe(&self) -> String {
        let mut strings = Vec::new();
        let mut fully_revealed = true;
        for card in self.cards.iter() {
            let symbol = &card.value;
            assert_ne!(matches!(symbol, CardSymbol::JOKER), true);
            if card.is_revealed() {
                strings.push(symbol.to_str().to_string());
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
        let new_hand = Hand::from_cards(vec![card_1_0, card_1_1]);
        new_hand
    }

    pub(crate) fn update_state(&mut self) {
        let mut sum = 0;
        let mut number_of_aces = 0;
        for card in self.cards.iter() {
            let value = &card.value;
            assert_ne!(matches!(value, CardSymbol::JOKER), true);
            sum += blackjack_card_value(&value);
            if matches!(value, CardSymbol::ACE) {
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

        if matches!(self.state, HandState::UNDEFINED) {
            self.state = HandState::from_value(sum);
        }
    }

    pub(crate) fn prompt_user_action(&self) -> UserAction {
        loop {
            if self.splitable() {
                match read_stdin_str!("STAY or HIT or SPLIT? [s/h/x]:")
                    .to_lowercase()
                    .as_str()
                {
                    "h" | "hit" => {
                        return UserAction::HIT;
                    }
                    "s" | "stay" => {
                        return UserAction::STAY;
                    }
                    "x" | "split" => {
                        return UserAction::SPLIT;
                    }
                    _ => {
                        println!("Invalid command.");
                        continue;
                    }
                }
            } else {
                match read_stdin_str!("STAY or HIT? [s/h]:")
                    .to_lowercase()
                    .as_str()
                {
                    "h" | "hit" => {
                        return UserAction::HIT;
                    }
                    "s" | "stay" => {
                        return UserAction::STAY;
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
