use std::collections::VecDeque;
use std::option::Option::Some;
use std::result::Result::Ok;

use crate::cards::{Card, Deck};
use crate::players::Player;

mod cards;
mod players;

struct Game {}

struct Round {}

fn main() {
    let number_of_players = 1;
    let number_of_actors = number_of_players + 1;

    if number_of_actors != 2 {
        unimplemented!("multi-player game is not implemented yet!")
    }

    let mut deck = Deck::new();

    println!("new deck with {} cards in it", deck.cards.len());

    deck.shuffle();

    let dealer = Player::new_dealer();
    let user = Player::new();
    let mut actors = vec![user, dealer];
    println!("{:?}", actors);

    let mut cards: VecDeque<Card> = deck.cards;
    for _ in 0..2 {
        for actor in actors.iter_mut() {
            match cards.pop_front() {
                Some(card) => actor.deal_card(card),
                _None => unreachable!()
            };
        }
    }

    println!("{} cards left in deck", cards.len());
    println!("{:?}", actors);

    println!("user hand:");
    actors.get(0).unwrap().hands[0].describe();

    println!("dealer hand:");
    actors.get(1).unwrap().hands[0].describe();
}
