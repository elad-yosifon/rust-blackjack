use std::borrow::ToOwned;
use std::clone::Clone;
use std::collections::VecDeque;
use std::io::stdin;
use std::iter::Iterator;
use std::option::Option::Some;
use std::process::{exit, Stdio};
use std::result::Result::{Err, Ok};
use std::string::String;

use crate::blackjack::{blackjack_calculate_hand, blackjack_play_round};
use crate::cards::{Card, Deck};
use crate::cards::CardSymbol::{ACE, JACK, KING, NINE};
use crate::cards::Suit::DIAMOND;
use crate::io::read_line;
use crate::players::{Hand, Player};
use crate::players::PlayerRole::PLAYER;

mod cards;
mod players;
mod blackjack;
mod io;

struct Game {}

struct Round {}

fn main() {

    let number_of_players = 1;
    let number_of_actors = number_of_players + 1;

    if number_of_actors != 2 {
        unimplemented!("multi-player game is not implemented yet!")
    }

    loop {
        blackjack_play_round(number_of_actors);

        match read_line("Another round? [y/n]").trim() {
            "y" | "yes" => continue,
            _ => {
                println!("Thanks for playing, bye :)");
                exit(0);
            }
        }
    }
}
