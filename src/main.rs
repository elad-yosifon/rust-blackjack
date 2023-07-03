#![feature(exclusive_range_pattern)]

use std::process::exit;
use std::vec;
use gameplay::round::blackjack_round;
use crate::gameplay::game::Game;


mod gameplay;
mod cards;
mod macros;

fn main() {

    let minimum_bet = 10;
    let number_of_players = read_stdin!("player count?", usize);

    if 1 > number_of_players || number_of_players > 4 {
        println!("Game can start with 1-4 players");
        exit(-1);
    }

    let player_scores = vec![minimum_bet * 10; number_of_players];

    let player_names = vec![""; number_of_players]
        .iter()
        .enumerate()
        .map(|(idx, _)| format!("User_{}", idx+1))
        .collect();

    let mut game = Game {
        current_round: blackjack_round(number_of_players, minimum_bet),
        player_scores,
        player_names,
    };


    loop {

        game.current_round.play();

        simulate_think!(1);
        game.judge_current_round();

        simulate_think!(2);
        game.print_player_scores();

        loop {
            match read_stdin_str!("\nAnother round? [y/n]:").to_lowercase().as_str() {
                "y" | "yes" => {

                    game.current_round = blackjack_round(number_of_players, minimum_bet);
                    break
                },
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
