use std::collections::VecDeque;
use std::iter::Iterator;
use std::option::Option::Some;
use std::result::Result::Ok;

use crate::blackjack::blackjack_calculate_hand;
use crate::cards::{Card, Deck};
use crate::cards::CardSymbol::{ACE, JACK, KING, NINE};
use crate::cards::Suit::DIAMOND;
use crate::players::{Hand, Player};

mod cards;
mod players;
mod blackjack;

struct Game {}

struct Round {}

fn main() {
    let number_of_players = 1;
    let number_of_actors = number_of_players + 1;

    if number_of_actors != 2 {
        unimplemented!("multi-player game is not implemented yet!")
    }

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

    let user_hand = &actors.get(0).unwrap().hands[0];
    let (sum, description) = blackjack_calculate_hand(user_hand);
    println!("user hand: {}", description);

    let dealer_hand = &actors.get(1).unwrap().hands[0];
    let (sum, description) = blackjack_calculate_hand(dealer_hand);
    println!("dealer hand: {}", description);

    //
    // let mut hand = Hand::from_cards(vec![
    //     Card { suit: DIAMOND, value: ACE },
    //     Card { suit: DIAMOND, value: ACE },
    //     Card { suit: DIAMOND, value: KING },
    // ]);
    // let (sum, description) = blackjack_calculate_hand(&hand);
    // println!("A A K hand: {}", description);
}
