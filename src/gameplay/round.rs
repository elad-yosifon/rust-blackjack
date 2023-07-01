use std::collections::VecDeque;
use crate::cards::deck::Deck;
use crate::gameplay::player::{Player, PlayerRole};
use crate::{at, simulate_think};
use crate::gameplay::blackjack::UserAction;
use crate::gameplay::hand::{Hand, HandState};

pub struct Round {
    pub deck: Deck,
    pub actors: Vec<Player>,
}

impl Round {
    pub fn play(&mut self) {
        self.deal_initial_cards();
        self.setup_dealer();

        self.update();

        let mut player_cursor = 0;
        let mut hand_cursor = 0;

        loop {
            let player = at!(mut self.actors, player_cursor);
            match player.role {
                PlayerRole::DEALER => {
                    break;
                }
                PlayerRole::PLAYER => {
                    let hand_count = player.hands.len();
                    if hand_cursor >= hand_count {
                        hand_cursor = 0;
                        player_cursor += 1;
                        continue;
                    }

                    loop {
                        let hand = player.hand_at_mut(hand_cursor);
                        match hand.state {
                            HandState::FINISHED => {
                                hand_cursor += 1;
                                break;
                            }
                            HandState::BUST => {
                                println!("Hand --> BUST \n");
                                hand_cursor += 1;
                                break;
                            }
                            HandState::BLACKJACK => {
                                println!("Hand --> BlackJack! \n");
                                hand_cursor += 1;
                                break;
                            }
                            HandState::UNDEFINED => {
                                match hand.prompt_user_action() {
                                    UserAction::HIT => {
                                        println!("Hand --> HIT \n");
                                        hand.deal_card(self.deck.draw_card());
                                        self.update();
                                    }
                                    UserAction::SPLIT => {
                                        println!("Hand --> SPLIT \n");
                                        let new_hand =
                                            hand.split(self.deck.draw_card(), self.deck.draw_card());
                                        player.hands.insert(hand_cursor + 1, new_hand);
                                        self.update();
                                    }
                                    UserAction::STAY => {
                                        println!("Hand --> STAY \n");
                                        hand.state = HandState::FINISHED;
                                        hand_cursor += 1;
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }

        simulate_think!(2);

        self.dealer_hand_mut()
            .card_at_mut(1)
            .reveal();

        self.update();

        simulate_think!(2);

        loop {
            let dealer_hand = self.dealer_hand();
            match dealer_hand.state {
                HandState::FINISHED => unreachable!(),
                HandState::BUST => {
                    println!("Dealer --> BUST \n");
                    break;
                }
                HandState::BLACKJACK => {
                    println!("Dealer --> BLACKJACK! \n");
                    break;
                }
                HandState::UNDEFINED => match self.dealer_hand().sum {
                    1..17 => {
                        println!("Dealer --> HIT \n");
                        let card = self.deck.draw_card();
                        self.dealer_hand_mut().deal_card(card);
                        simulate_think!(2);
                        self.update();
                    }
                    17..21 => {
                        println!("Dealer --> STAY \n");
                        self.dealer_hand_mut().state = HandState::FINISHED;
                        break;
                    }
                    _ => unreachable!(),
                },
            }
        }
    }
}

impl Round {

    fn dealer_hand(&self) -> &Hand {
        self.dealer().hand_at(0)
    }
    fn dealer_hand_mut(&mut self) -> &mut Hand {
        self.dealer_mut().hand_at_mut(0)
    }

    fn dealer(&self) -> &Player {
        self.actors.last().unwrap()
    }

    fn dealer_mut(&mut self) -> &mut Player {
        self.actors.last_mut().unwrap()
    }

    fn update(&mut self) {
        self.update_state();
        self.print_current_game_state();
    }

    fn print_current_game_state(&self) {
        for player in self.actors.iter() {
            for hand in player.hands.iter() {
                println!(
                    "{} hand: {}",
                    player.role.as_str(),
                    hand.describe()
                );
            }
        }
    }

    fn setup_dealer(&mut self) {
        let dealer = self.dealer_mut();
        dealer.role = PlayerRole::DEALER;
        dealer.hand_at_mut(0).card_at_mut(1).unreveal();
    }

    fn deal_initial_cards(&mut self) {
        let mut hands = VecDeque::with_capacity(self.actors.capacity());
        for _ in 0..self.actors.capacity() {
            hands.push_back(Hand::new());
        }

        while !hands.is_empty() {
            let mut hand = hands.pop_front().unwrap();
            hand.deal_card(self.deck.draw_card());
            hand.deal_card(self.deck.draw_card());
            self.actors.push(Player::new(hand))
        }
    }

    fn update_state(&mut self) {
        for player in self.actors.iter_mut() {
            for hand in player.hands.iter_mut() {
                hand.update_state();
            }
        }
    }
}

pub fn blackjack_round(number_of_user_players: usize) -> Round {
    Round {
        actors: Vec::with_capacity(number_of_user_players + 1),
        deck: Deck::new_shuffled(),
    }
}


pub fn blackjack_judge_round(round: &Round) {
    let dealer_hand = round.dealer_hand();
    for actor_idx in 0..round.actors.len() - 1 {
        let actor_name = format!("User_{}", actor_idx + 1);
        at!(round.actors, actor_idx).hands.iter().for_each(|user_hand| {
            match (&user_hand.state, &dealer_hand.state) {
                (HandState::FINISHED, HandState::FINISHED) => {
                    if user_hand.sum == dealer_hand.sum {
                        println!(
                            "Draw <-- {} and Dealer got same value={}!",
                            actor_name, dealer_hand.sum
                        );
                    } else if user_hand.sum > dealer_hand.sum {
                        println!(
                            "{} Won <-- value={} > Dealer value={}",
                            actor_name, user_hand.sum, dealer_hand.sum
                        );
                    } else {
                        println!(
                            "{} Lost <-- value={} < Dealer value={}",
                            actor_name, user_hand.sum, dealer_hand.sum
                        );
                    }
                }
                (HandState::BLACKJACK, HandState::BLACKJACK) => {
                    println!("Draw <-- {} and Dealer both got BLACKJACK!", actor_name);
                }
                (HandState::BLACKJACK, _) => {
                    println!("{} Won <-- {0} have BLACKJACK!", actor_name);
                }
                (HandState::FINISHED, HandState::BUST) => {
                    println!("{} Won <-- Dealer is BUSTED!", actor_name);
                }
                (HandState::BUST, _) => {
                    println!("{} Lost <-- {0} is BUSTED!", actor_name);
                }
                (_, HandState::BLACKJACK) => {
                    println!("{} Lost <-- Dealer has BLACKJACK!", actor_name);
                }
                (HandState::UNDEFINED, _) | (_, HandState::UNDEFINED) => unreachable!(),
            }
        });
    }
}
