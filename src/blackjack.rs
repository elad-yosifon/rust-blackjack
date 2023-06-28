use std::clone::Clone;
use std::collections::VecDeque;
use std::iter::Iterator;
use std::option::Option::Some;
use std::string::{String, ToString};
use std::vec::Vec;

use crate::cards::{Card, CardSymbol, Deck};
use crate::io::read_line;
use crate::players::{Hand, Player};

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

pub fn blackjack_play_round(number_of_actors: usize) {

    let mut deck = Deck::new();

    // println!("new deck with {} cards in it", deck.cards.len());

    deck.shuffle();

    let dealer = Player::new_dealer();
    let user = Player::new();
    let mut actors = vec![user, dealer];
    // println!("{:?}", actors);

    let mut cards: VecDeque<Card> = deck.cards;
    for i in 0..2 {
        for (idx, actor) in actors.iter_mut().enumerate() {
            let mut card: Card = match cards.pop_front() {
                Some(card) => card,
                _None => unreachable!()
            };
            if i == 1 && idx == (number_of_actors - 1) {
                card.unreveal()
            }
            actor.deal_card(card)
        }
    }

    // println!("{} cards left in deck", cards.len());
    // println!("{:?}", actors);

    loop {
        let user = actors.get_mut(0).unwrap();
        let user_hand = &user.hands[0];
        let (sum, description) = blackjack_calculate_hand(user_hand);
        println!("user hand: {}", description);

        let dealer = actors.get_mut(1).unwrap();
        let dealer_hand = &dealer.hands[0];
        let (sum, description) = blackjack_calculate_hand(dealer_hand);
        println!("dealer hand: {}", description);

        let action = user_action();
        match action {
            UserAction::HIT => {
                let user = actors.get_mut(0).unwrap();
                user.deal_card(match cards.pop_front() {
                    Some(card) => card,
                    _None => unreachable!()
                })
            }
            UserAction::STAY => {
                break;
            }
        }
    }

    // let mut hand = Hand::from_cards(vec![
    //     Card { suit: DIAMOND, value: ACE },
    //     Card { suit: DIAMOND, value: ACE },
    //     Card { suit: DIAMOND, value: KING },
    // ]);
    // let (sum, description) = blackjack_calculate_hand(&hand);
    // println!("A A K hand: {}", description);
}

enum UserAction {
    HIT,
    STAY
}

fn user_action() -> UserAction {
    loop {
        let input = read_line(format!("Hit(h)|Stay(s)?").as_str());
        match input.trim() {
            "h" | "hit" => {
                return UserAction::HIT;
            },
            "s" | "stay" => {
                return UserAction::STAY;
            },
            _ => {
                println!("Invalid command.");
                continue
            }
        }
    }
}