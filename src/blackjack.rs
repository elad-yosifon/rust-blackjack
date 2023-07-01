use std::collections::VecDeque;
use std::iter::Iterator;
use std::string::{String, ToString};
use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

use crate::{actor_at, at};
use crate::{draw_card, simulate_think};
use crate::{read_stdin_str, read_stdin_string};
use crate::cards::{CardSymbol, Deck};
use crate::players::{Hand, HandState, Player, PlayerRole};

pub fn blackjack_card_value(card_symbol: &CardSymbol) -> i32 {
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

    hand.sum = sum;

    if matches!(hand.state, HandState::UNDEFINED) {
        hand.state = HandState::from_value(hand.sum);
    }
}

fn blackjack_describe_hand(hand: &Hand) -> String {
    let mut strings = Vec::new();
    let mut fully_revealed = true;
    for card in hand.cards.iter() {
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
        hand_str.push_str(format!(" = {}", hand.sum).as_str());
    } else {
        hand_str.push_str(" = ?");
    }

    hand_str
}

pub fn blackjack_judge_round(actors: &Vec<Player>) {
    let dealer_hand = actors.last().unwrap().hand_at(0);
    for actor_idx in 0..actors.len() - 1 {
        let actor_name = format!("User_{}", actor_idx + 1);
        actor_at!(actors, actor_idx)
            .hands
            .iter()
            .for_each(|user_hand| match (&user_hand.state, &dealer_hand.state) {
                (HandState::FINISHED, HandState::FINISHED) => {
                    if user_hand.sum == dealer_hand.sum {
                        println!(
                            "Draw <-- {} and Dealer got same value={}!",
                            actor_name, dealer_hand.sum
                        );
                    } else if user_hand.sum > dealer_hand.sum {
                        println!(
                            "{} Won <-- value={} > Dealer value={}",
                            actor_name, user_hand.sum, dealer_hand.sum
                        );
                    } else {
                        println!(
                            "{} Lost <-- value={} < Dealer value={}",
                            actor_name, user_hand.sum, dealer_hand.sum
                        );
                    }
                }
                (HandState::BLACKJACK, HandState::BLACKJACK) => {
                    println!("Draw <-- {} and Dealer both got BLACKJACK!", actor_name);
                }
                (HandState::BLACKJACK, _) => {
                    println!("{} Won <-- {0} have BLACKJACK!", actor_name);
                }
                (HandState::FINISHED, HandState::BUST) => {
                    println!("{} Won <-- Dealer is BUSTED!", actor_name);
                }
                (HandState::BUST, _) => {
                    println!("{} Lost <-- {0} is BUSTED!", actor_name);
                }
                (_, HandState::BLACKJACK) => {
                    println!("{} Lost <-- Dealer has BLACKJACK!", actor_name);
                }
                (HandState::UNDEFINED, _) | (_, HandState::UNDEFINED) => unreachable!(),
            });
    }
}

#[allow(dead_code)]
struct Game {}

pub struct Round {
    pub actors: Vec<Player>,
}

pub fn blackjack_play_round(number_of_user_players: usize) -> Round {
    let mut actors = Vec::with_capacity(number_of_user_players + 1);
    let mut deck = Deck::new_shuffled();

    blackjack_deal_initial_cards(&mut actors, &mut deck);
    blackjack_setup_dealer(&mut actors);

    blackjack_update_current_game_state(&mut actors);
    blackjack_print_current_game_state(&actors);

    let mut player_cursor = 0;
    let mut hand_cursor = 0;

    loop {
        let player = actor_at!(mut actors, player_cursor);
        match player.role {
            PlayerRole::DEALER => {
                break;
            }
            PlayerRole::PLAYER => {
                let hand_count = player.hands.len();
                if hand_cursor >= hand_count {
                    hand_cursor = 0;
                    player_cursor += 1;
                    continue;
                }

                loop {
                    let hand = at!(mut player.hands, hand_cursor);
                    match hand.state {
                        HandState::FINISHED => {
                            hand_cursor += 1;
                            break;
                        }
                        HandState::BUST => {
                            println!("Hand --> BUST \n");
                            hand_cursor += 1;
                            break;
                        }
                        HandState::BLACKJACK => {
                            println!("Hand --> BlackJack! \n");
                            hand_cursor += 1;
                            break;
                        }
                        HandState::UNDEFINED => {
                            match blackjack_prompt_user_action(hand) {
                                UserAction::HIT => {
                                    println!("Hand --> HIT \n");
                                    hand.deal_card(draw_card!(deck));
                                    blackjack_update_current_game_state(&mut actors);
                                    blackjack_print_current_game_state(&actors);
                                }
                                UserAction::SPLIT => {
                                    println!("Hand --> SPLIT \n");
                                    let new_hand = hand.split(draw_card!(deck), draw_card!(deck));
                                    player.hands.insert(hand_cursor + 1, new_hand);
                                    blackjack_update_current_game_state(&mut actors);
                                    blackjack_print_current_game_state(&actors);
                                }
                                UserAction::STAY => {
                                    println!("Hand --> STAY \n");
                                    hand.state = HandState::FINISHED;
                                    hand_cursor += 1;
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    simulate_think!(2);

    actor_at!(mut actors, 1)
        .hand_at_mut(0)
        .card_at_mut(1)
        .reveal();

    blackjack_update_current_game_state(&mut actors);
    blackjack_print_current_game_state(&actors);

    simulate_think!(2);

    loop {
        let dealer_hand = actor_at!(mut actors, 1).hand_at_mut(0);
        match dealer_hand.state {
            HandState::FINISHED => unreachable!(),
            HandState::BUST => {
                println!("Dealer --> BUST \n");
                break;
            }
            HandState::BLACKJACK => {
                println!("Dealer --> BLACKJACK! \n");
                break;
            }
            HandState::UNDEFINED => match dealer_hand.sum {
                1..17 => {
                    println!("Dealer --> HIT \n");
                    dealer_hand.deal_card(draw_card!(deck));

                    simulate_think!(2);
                    blackjack_update_current_game_state(&mut actors);
                    blackjack_print_current_game_state(&actors);
                }
                17..21 => {
                    println!("Dealer --> STAY \n");
                    dealer_hand.state = HandState::FINISHED;
                    break;
                }
                _ => unreachable!(),
            },
        }
    }

    Round { actors }
}

fn blackjack_deal_initial_cards(actors: &mut Vec<Player>, deck: &mut Deck) {
    let mut hands = VecDeque::with_capacity(actors.capacity());
    for _ in 0..actors.capacity() {
        hands.push_back(Hand::new());
    }

    while !hands.is_empty() {
        let mut hand = hands.pop_front().unwrap();
        hand.deal_card(draw_card!(deck));
        hand.deal_card(draw_card!(deck));
        actors.push(Player::new(hand))
    }
}

fn blackjack_setup_dealer(actors: &mut Vec<Player>) {
    let dealer = actor_at!(mut actors, 1);
    dealer.role = PlayerRole::DEALER;
    dealer.hand_at_mut(0).card_at_mut(1).unreveal();
}

fn blackjack_print_current_game_state(actors: &Vec<Player>) {
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

fn blackjack_update_current_game_state(actors: &mut Vec<Player>) {
    for player in actors.iter_mut() {
        for hand in player.hands.iter_mut() {
            blackjack_update_hand_state(hand);
        }
    }
}

enum UserAction {
    HIT,
    STAY,
    SPLIT,
}

fn blackjack_prompt_user_action(hand: &mut Hand) -> UserAction {
    loop {
        if hand.splitable() {
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
