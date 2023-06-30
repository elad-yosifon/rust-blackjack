#![feature(exclusive_range_pattern)]

use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use crate::blackjack::{blackjack_judge_round, blackjack_play_round};
use crate::players::Player;

mod blackjack;
mod cards;
mod macros;
mod players;

#[allow(dead_code)]
struct Game {}

#[allow(dead_code)]
struct Round {}

fn main() {
    let number_of_players = 1;
    let number_of_actors = number_of_players + 1;

    if number_of_actors != 2 {
        unimplemented!("multi-player game is not implemented yet!")
    }

    loop {
        let dealer = Player::new_dealer();
        let user = Player::new();
        let mut actors = vec![user, dealer];
        // println!("{:?}", actors);

        blackjack_play_round(&mut actors);
        simulate_think!(2);

        blackjack_judge_round(&actors);
        simulate_think!(2);

        match read_stdin_str!("Another round? [y/n]") {
            "y" | "yes" => continue,
            _ => {
                println!("Thanks for playing, bye :)");
                exit(0);
            }
        }
    }
}
