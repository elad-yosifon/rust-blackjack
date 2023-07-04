use crate::at;
use crate::gameplay::hand::{Hand, HandState};
use crate::gameplay::round::Round;

#[allow(dead_code)]
pub struct Game {
    pub player_scores: Vec<i32>,
    pub player_names: Vec<String>,
}

impl Game {
    pub fn print_player_scores(&self) {
        println!();
        println!("Scores:");
        println!("=======");
        self.player_names.iter().enumerate().for_each(|(i, name)| {
            println!("{}: {} coins", name, at!(self.player_scores, i));
        });
    }
}

fn print_hand_result(result: &HandResult, player_name: &String, coins: &i32) {
    match result {
        HandResult::AutoWin => println!("{} : +{:2} coins --> AUTO BLACKJACK!", player_name, coins),
        HandResult::Win => println!("{} : +{:2} coins --> WON", player_name, coins),
        HandResult::Draw => println!("{} : +{:2} coins --> DRAW", player_name, coins),
        HandResult::Loss => println!("{} : -{:2} coins --> LOSS", player_name, -coins),
    }
}

impl Game {
    pub fn judge_round(&mut self, round: &Round) {
        let i = round.actors.len();
        for actor_idx in 0..(i - 1) {
            let actor = at!(round.actors, actor_idx);
            for hand in &actor.hands {
                let hand_result = calculate_hand_result(hand, round.dealer_hand());
                let coins = match hand_result {
                    HandResult::AutoWin => (3 * at!(round.actor_bets, actor_idx)) / 2,
                    HandResult::Win => *at!(round.actor_bets, actor_idx),
                    HandResult::Loss => -at!(round.actor_bets, actor_idx),
                    HandResult::Draw => 0,
                };
                print_hand_result(&hand_result, at!(self.player_names, actor_idx), &coins);

                let score = at!(mut self.player_scores, actor_idx);
                *score += coins
            }
        }
    }
}

enum HandResult {
    AutoWin,
    Win,
    Loss,
    Draw,
}

fn calculate_hand_result(user_hand: &Hand, dealer_hand: &Hand) -> HandResult {
    match (&user_hand.state, &dealer_hand.state) {
        (HandState::Finished, HandState::Finished) => match user_hand.sum - dealer_hand.sum {
            ..0 => HandResult::Loss,
            0 => HandResult::Draw,
            1.. => HandResult::Win,
        },
        (HandState::Blackjack, _) => {
            if user_hand.cards.len() == 2 {
                HandResult::AutoWin
            } else if matches!(dealer_hand.state, HandState::Blackjack) {
                HandResult::Draw
            } else {
                HandResult::Win
            }
        }
        (HandState::Finished, HandState::Bust) => HandResult::Win,
        (HandState::Bust, _) | (_, HandState::Blackjack) => HandResult::Loss,
        (HandState::Undefined, _) | (_, HandState::Undefined) => unreachable!(),
    }
}
