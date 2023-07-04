#![feature(exclusive_range_pattern)]

use std::process::exit;
use std::vec;

use gameplay::round::blackjack_round;

use crate::gameplay::game::Game;

mod cards;
mod gameplay;
mod macros;

fn main() {
    let minimum_bet = 10;
    let number_of_players = match take_stdin_key!("Number of players? [1/2/3]", '1', '2', '3') {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        _ => 0,
    };

    if !(1..=3).contains(&number_of_players) {
        println!("Game can start with 1-3 players");
        exit(-1);
    } else {
        println!("{}", number_of_players);
    }

    let player_scores = vec![minimum_bet * 10; number_of_players];

    let player_names = vec![""; number_of_players]
        .iter()
        .enumerate()
        .map(|(idx, _)| format!("User_{}", idx + 1))
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
            println!();
            match take_stdin_key!("Another round? [y/n]:", 'y', 'n') {
                'y' => {
                    game.current_round = blackjack_round(number_of_players, minimum_bet);
                    break;
                }
                'n' => {
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
