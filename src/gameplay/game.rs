use crate::at;
use crate::gameplay::hand::{Hand, HandState};
use crate::gameplay::round::Round;

#[allow(dead_code)]
pub struct Game {
    pub current_round: Round,
    pub player_scores: Vec<i32>,
    pub player_names: Vec<String>,
}

impl Game {
    pub fn print_player_scores(&self) {
        println!();
        println!("Scores:");
        println!("=======");
        self.player_names.iter().enumerate().for_each(|(i,name)| {
            println!("{}: {} coins", name, at!(self.player_scores, i));
        });
    }
}

impl Game {
    pub fn judge_current_round(&mut self) {
        let i = &self.current_round.actors.len();
        for actor_idx in 0..(i - 1) {
            let actor = at!(&self.current_round.actors, actor_idx);
            for hand in &actor.hands {
                let act = calculate_score_result(hand, self.current_round.dealer_hand());
                let score = at!(mut self.player_scores, actor_idx);
                let round = &self.current_round;
                *score += match act {
                    ScoreResult::AutoWin => {
                        let coins = (3 * at!(round.actor_bets, actor_idx)) / 2;
                        println!(
                            "{} got immediate BLACKJACK:  +{} coins",
                            at!(self.player_names, actor_idx),
                            coins
                        );
                        coins
                    }
                    ScoreResult::Win => {
                        let coins = at!(round.actor_bets, actor_idx).clone();
                        println!(
                            "{} WON:  +{} coins",
                            at!(self.player_names, actor_idx),
                            coins
                        );
                        coins
                    }
                    ScoreResult::Loss => {
                        let coins = at!(round.actor_bets, actor_idx);
                        println!(
                            "{} LOST:  -{} coins",
                            at!(self.player_names, actor_idx),
                            coins
                        );
                        -coins
                    }
                    ScoreResult::Draw => {
                        println!("{} DRAW:  +{} coins", at!(self.player_names, actor_idx), 0);
                        0
                    }
                }
            }
        }
    }
}

enum ScoreResult {
    AutoWin,
    Win,
    Loss,
    Draw,
}
fn calculate_score_result(user_hand: &Hand, dealer_hand: &Hand) -> ScoreResult {
    match (&user_hand.state, &dealer_hand.state) {
        (HandState::FINISHED, HandState::FINISHED) => match user_hand.sum - dealer_hand.sum {
            ..0 => ScoreResult::Loss,
            0 => ScoreResult::Draw,
            1.. => ScoreResult::Win,
        },
        (HandState::BLACKJACK, _) => {
            if user_hand.cards.len() == 2 {
                return ScoreResult::AutoWin;
            } else {
                return if matches!(dealer_hand.state, HandState::BLACKJACK) {
                    ScoreResult::Draw
                } else {
                    ScoreResult::Win
                };
            };
        }
        (HandState::FINISHED, HandState::BUST) => {
            return ScoreResult::Win;
        }
        (HandState::BUST, _) | (_, HandState::BLACKJACK) => {
            return ScoreResult::Loss;
        }
        (HandState::UNDEFINED, _) | (_, HandState::UNDEFINED) => unreachable!(),
    }
}
