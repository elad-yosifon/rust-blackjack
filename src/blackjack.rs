use std::iter::Iterator;
use std::string::{String, ToString};
use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

use crate::cards::{CardSymbol, Deck};
use crate::players::{Hand, HandState, Player};
use crate::{actor_at, deal_card, read_stdin_str, read_stdin_string, simulate_think};

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

fn blackjack_update_hand_state(hand: &mut Hand) {
    let mut sum = 0;
    let mut number_of_aces = 0;
    for card in hand.cards.iter() {
        let value = card.value;
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
    hand.sum = sum;
    hand.state = HandState::from_value(hand.sum);
}

fn blackjack_describe_hand(hand: &Hand) -> String {
    let mut strings = Vec::new();
    let mut fully_revealed = true;
    for card in hand.cards.iter() {
        let symbol = card.value;
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
        hand_str.push_str(format!(" = {}", hand.sum).as_str());
    } else {
        hand_str.push_str(" = ?");
    }

    hand_str
}

pub fn blackjack_judge_round(actors: &Vec<Player>) {
    let user_hand = actor_at!(actors, 0).hand_at(0);
    let dealer_hand = actor_at!(actors, 1).hand_at(0);
    let scores_tup = (&user_hand.state, &dealer_hand.state);

    match scores_tup {
        (HandState::FINISHED, HandState::FINISHED) => {
            if user_hand.sum == dealer_hand.sum {
                println!(
                    "Draw <-- user and dealer got same value {}!",
                    dealer_hand.sum
                );
            } else if user_hand.sum > dealer_hand.sum {
                println!(
                    "User Won <-- user value {} > dealer value {} ",
                    user_hand.sum, dealer_hand.sum
                );
            } else {
                println!(
                    "User Lost <-- user value {} < dealer value {} ",
                    user_hand.sum, dealer_hand.sum
                );
            }
        }
        (HandState::BLACKJACK, HandState::BLACKJACK) => {
            println!("Draw <-- user and dealer got BLACKJACK!");
        }
        (HandState::BLACKJACK, _) => {
            println!("User Won <-- user have BLACKJACK!");
        }
        (HandState::FINISHED, HandState::BUST) => {
            println!("User Won <-- dear is BUSTED!");
        }
        (HandState::BUST, _) => {
            println!("User Lost <-- user is BUSTED!");
        }
        (_, HandState::BLACKJACK) => {
            println!("User Lost <-- dealer has BLACKJACK!");
        }
        (HandState::UNDEFINED, _) | (_, HandState::UNDEFINED) => unreachable!(),
    }
}

pub fn blackjack_play_round(actors: &mut Vec<Player>) {
    let mut deck = Deck::new();
    deck.shuffle();
    deal_cards(actors, &mut deck);

    loop {
        update_current_game_state(actors);
        print_current_game_state(&actors);

        let user_hand_sum = actor_at!(actors, 0).hand_at(0).sum;
        if user_hand_sum > 21 {
            println!("User --> BUST \n");
            break;
        } else if user_hand_sum == 21 {
            println!("User --> BlackJack! \n");
            break;
        }

        match user_action() {
            UserAction::HIT => {
                println!("User --> Hit \n");
                deal_card!(actor_at!(mut actors, 0), deck);
            }
            UserAction::STAY => {
                println!("User --> Stay \n");
                break;
            }
        }
    }

    simulate_think!(2);
    actor_at!(mut actors, 1)
        .hand_at_mut(0)
        .card_at_mut(1)
        .reveal();

    loop {
        update_current_game_state(actors);
        print_current_game_state(&actors);

        simulate_think!(2);

        let dealer = actor_at!(actors, 1);
        match dealer.hand_at(0).sum {
            1..17 => {
                println!("Dealer --> Hit \n");
                deal_card!(actor_at!(mut actors, 1), deck);
                continue;
            }
            17..21 => {
                println!("Dealer --> Stay \n");
                break;
            }
            21 => {
                println!("Dealer --> BLACKJACK! \n");
                break;
            }
            22.. => {
                println!("Dealer --> BUST \n");
                break;
            }
            _ => unreachable!(),
        }
    }
}

fn print_current_game_state(actors: &Vec<Player>) {
    for player in actors.iter() {
        for hand in player.hands.iter() {
            println!(
                "{} hand: {}",
                player.role.as_str(),
                blackjack_describe_hand(hand)
            );
        }
    }
}

fn deal_cards(actors: &mut Vec<Player>, deck: &mut Deck) {
    actors.iter_mut().for_each(|actor| deal_card!(actor, deck));
    actors.iter_mut().for_each(|actor| deal_card!(actor, deck));
    actor_at!(mut actors, 1)
        .hand_at_mut(0)
        .card_at_mut(1)
        .unreveal();
}

fn update_current_game_state(actors: &mut Vec<Player>) {
    for player in actors.iter_mut() {
        for hand in player.hands.iter_mut() {
            blackjack_update_hand_state(hand);
        }
    }
}

enum UserAction {
    HIT,
    STAY,
}

fn user_action() -> UserAction {
    loop {
        match read_stdin_str!("Hit(h)|Stay(s)?") {
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
