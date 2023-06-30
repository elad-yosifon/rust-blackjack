#![feature(exclusive_range_pattern)]

use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use crate::blackjack::{blackjack_judge_round, blackjack_play_round};

mod blackjack;
mod cards;
mod macros;
mod players;

fn main() {
    let number_of_players = 1;
    let number_of_actors = number_of_players + 1;

    if number_of_actors != 2 {
        unimplemented!("multi-player game is not implemented yet!")
    }

    loop {
        let round = blackjack_play_round(1);

        simulate_think!(1);
        blackjack_judge_round(&round.actors);

        simulate_think!(2);
        loop {
            match read_stdin_str!("\nAnother round? [y/n]:").to_lowercase().as_str() {
                "y" | "yes" => break,
                "n" | "no" => {
                    println!("Thanks for playing, bye :)");
                    exit(0);
                }
                _ => {
                    println!("Invalid command.");
                    continue;
                }
            }
        }
    }
}
