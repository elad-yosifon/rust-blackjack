use std::clone::Clone;
use std::collections::VecDeque;
use std::convert::From;
use std::iter::Iterator;
use std::option::Option::Some;
use std::string::{String, ToString};
use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

use crate::cards::{Card, CardSymbol, Deck};
use crate::io::read_line;
use crate::players::{Hand, HandState, Player};

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

fn blackjack_calculate_hand(hand: &Hand) -> i32 {
    let mut sum = 0;
    let mut number_of_aces = 0;

    for card in hand.cards.iter() {
        let value = card.value;
        assert_ne!(matches!(value,CardSymbol::JOKER), true);
        sum += blackjack_card_value(&value);
        if matches!(value,CardSymbol::ACE) {
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
    sum
}

fn blackjack_describe_hand(hand: &Hand) -> String {
    let mut strings = Vec::new();
    let mut fully_revealed = true;
    for card in hand.cards.iter() {
        let value = card.value;
        assert_ne!(matches!(value,CardSymbol::JOKER), true);
        if card.is_revealed() {
            strings.push(value.to_string());
        } else {
            fully_revealed = false;
            strings.push("X".to_string());
        }
    }

    let mut hand_str = strings.join(" + ");
    if fully_revealed {
        hand_str.push_str(format!(" = {}", hand.sum.to_string()).as_str());
    } else {
        hand_str.push_str(" = ?");
    }

    hand_str
}

pub fn blackjack_judge_round(actors: &Vec<Player>) {
    let dealer = actors.get(1).unwrap();
    let user = actors.get(0).unwrap();
    let user_hand = user.hands.get(0).unwrap();
    let dealer_hand = dealer.hands.get(0).unwrap();
    let scores_tup = (&user_hand.state, &dealer_hand.state);

    //TODO: support "immediate BJ"
    match scores_tup {
        (HandState::FINISHED, HandState::FINISHED) => {
            if user_hand.sum == dealer_hand.sum {
                println!("Draw <-- user and dealer got same value {}!", dealer_hand.sum);
            } else if user_hand.sum > dealer_hand.sum {
                println!("User Won <-- user value {} > dealer value {} ", user_hand.sum, dealer_hand.sum);
            } else {
                println!("User Lost <-- user value {} < dealer value {} ", user_hand.sum, dealer_hand.sum);
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

    // println!("new deck with {} cards in it", deck.cards.len());
    deck.shuffle();

    deal_cards(actors, &mut deck);

    // println!("{} cards left in deck", cards.len());
    // println!("{:?}", actors);

    loop {
        update_current_game_state(actors);
        print_current_game_state(&actors);

        let user_hand_sum = actors.get(0).unwrap().hands[0].sum;
        if user_hand_sum > 21 {
            println!("User --> BUST \n");
            break;
        } else if user_hand_sum == 21 {
            println!("User --> BlackJack! \n");
            break;
        }

        let action = user_action();
        match action {
            UserAction::HIT => {
                println!("User --> Hit \n");
                let user = actors.get_mut(0).unwrap();
                user.deal_card(match deck.cards.pop_front() {
                    Some(card) => card,
                    _None => unreachable!()
                })
            }
            UserAction::STAY => {
                println!("User --> Stay \n");
                break;
            }
        }
    }

    sleep(Duration::from_secs(2));

    let dealer = actors.get_mut(1).unwrap();
    let dealer_hand = &mut dealer.hands[0];
    dealer_hand.cards[1].reveal();

    loop {
        update_current_game_state(actors);
        print_current_game_state(&actors);

        sleep(Duration::from_secs(2));

        let dealer = actors.get_mut(1).unwrap();
        let dealer_hand = &mut dealer.hands[0];

        let dealer_sum = dealer_hand.sum;
        match dealer_sum {
            1..17 => {
                println!("Dealer --> Hit \n");
                dealer.deal_card(match deck.cards.pop_front() {
                    Some(card) => card,
                    _None => unreachable!()
                });
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
            _ => unreachable!()
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

fn print_current_game_state(actors: &Vec<Player>) {
    println!("dealer hand: {}", blackjack_describe_hand(&actors.get(1).unwrap().hands[0]));
    println!("user hand: {}", blackjack_describe_hand(&actors.get(0).unwrap().hands[0]));
}

fn deal_cards(actors: &mut Vec<Player>, deck: &mut Deck) {
    let number_of_actors = actors.len();
    let mut cards = &mut deck.cards;
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
}

fn update_current_game_state(actors: &mut Vec<Player>) {
    let user = actors.get_mut(0).unwrap();
    let mut user_hand = &mut user.hands[0];
    user_hand.sum = blackjack_calculate_hand(&user_hand);
    user_hand.state = HandState::from_value(user_hand.sum);


    let dealer = actors.get_mut(1).unwrap();
    let mut dealer_hand = &mut dealer.hands[0];
    dealer_hand.sum = blackjack_calculate_hand(&dealer_hand);
    dealer_hand.state = HandState::from_value(dealer_hand.sum);
}

enum UserAction {
    HIT,
    STAY,
}

fn user_action() -> UserAction {
    loop {
        let input = read_line(format!("Hit(h)|Stay(s)?").as_str());
        match input.trim() {
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