use std::clone::Clone;
use std::string::{String, ToString};
use std::vec::Vec;

use crate::cards::CardSymbol;
use crate::players::Hand;

fn blackjack_card_value(card_symbol: &CardSymbol) -> i32 {
    match card_symbol {
        CardSymbol::TWO => 2,
        CardSymbol::THREE => 3,
        CardSymbol::FOUR => 4,
        CardSymbol::FIVE => 5,
        CardSymbol::SIX => 6,
        CardSymbol::SEVEN => 7,
        CardSymbol::EIGHT => 8,
        CardSymbol::NINE => 9,
        CardSymbol::TEN | CardSymbol::JACK | CardSymbol::QUEEN | CardSymbol::KING => 10,
        CardSymbol::ACE => 11,
        CardSymbol::JOKER => 0,
    }
}

pub fn blackjack_calculate_hand(hand: &Hand) -> (i32, String) {
    let mut hand_str = String::new();
    let mut strings = Vec::new();
    let mut sum = 0;
    let mut number_of_aces = 0;
    let mut fully_revealed = true;
    for card in hand.cards.iter() {
        let value = card.value;
        assert_ne!(matches!(value,CardSymbol::JOKER), true);
        if matches!(value,CardSymbol::ACE) {
            number_of_aces += 1;
        }
        sum += blackjack_card_value(&value);
        if card.is_revealed() {
            strings.push(value.to_string());
        } else {
            fully_revealed = false;
            strings.push("X".to_string());
        }
    }

    hand_str.push_str(strings.join(" + ").as_str());

    while number_of_aces > 0 {
        if sum <= 21 {
            break;
        }
        sum -= 10;
        number_of_aces -= 1;
    }

    hand_str.push_str(format!(" = {}", if fully_revealed { sum.to_string() } else { "?".to_string() }).as_str());
    (sum, hand_str)
}