use crate::{at, simulate_think};
use crate::cards::deck::Deck;
use crate::gameplay::actor::{Actor, ActorRole};
use crate::gameplay::blackjack::UserAction;
use crate::gameplay::hand::{Hand, HandState};

pub struct Round {
    pub deck: Deck,
    pub actors: Vec<Actor>,
    pub actor_bets: Vec<i32>,
}

impl Round {
    pub fn play(&mut self) {
        self.deal_initial_cards();
        self.setup_dealer();

        self.update();

        let mut actor_cursor = 0;
        let mut hand_cursor = 0;

        loop {
            let actor_name = at!(self.actors, actor_cursor).name.clone();
            let actor = at!(mut self.actors, actor_cursor);
            match actor.role {
                ActorRole::Dealer => {
                    break;
                }
                ActorRole::Player => {
                    let hand_count = actor.hands.len();
                    if hand_cursor >= hand_count {
                        hand_cursor = 0;
                        actor_cursor += 1;
                        continue;
                    }

                    let hand = actor.hand_at_mut(hand_cursor);
                    match hand.state {
                        HandState::Finished => {
                            hand_cursor += 1;
                        }
                        HandState::Bust => {
                            println!("Hand --> BUST \n");
                            hand_cursor += 1;
                        }
                        HandState::Blackjack => {
                            println!("Hand --> BlackJack! \n");
                            hand_cursor += 1;
                        }
                        HandState::Undefined => {
                            match hand.prompt_user_action(&actor_name, hand_cursor) {
                                UserAction::Hit => {
                                    println!("Hand --> HIT \n");
                                    hand.deal_card(self.deck.draw_card());
                                    self.update();
                                }
                                UserAction::Split => {
                                    println!("Hand --> SPLIT \n");
                                    let new_hand =
                                        hand.split(self.deck.draw_card(), self.deck.draw_card());
                                    actor.hands.insert(hand_cursor + 1, new_hand);
                                    self.update();
                                }
                                UserAction::Stay => {
                                    println!("Hand --> STAY \n");
                                    hand.state = HandState::Finished;
                                    hand_cursor += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        simulate_think!(2);

        self.dealer_hand_mut().card_at_mut(1).reveal();

        self.update();

        simulate_think!(2);

        loop {
            let dealer_hand = self.dealer_hand();
            match dealer_hand.state {
                HandState::Finished => unreachable!(),
                HandState::Bust => {
                    println!("Dealer --> BUST \n");
                    break;
                }
                HandState::Blackjack => {
                    println!("Dealer --> BLACKJACK! \n");
                    break;
                }
                HandState::Undefined => match self.dealer_hand().sum {
                    1..17 => {
                        println!("Dealer --> HIT \n");
                        let card = self.deck.draw_card();
                        self.dealer_hand_mut().deal_card(card);
                        simulate_think!(2);
                        self.update();
                    }
                    17..21 => {
                        println!("Dealer --> STAY \n");
                        self.dealer_hand_mut().state = HandState::Finished;
                        break;
                    }
                    _ => unreachable!(),
                },
            }
        }
    }

    pub(crate) fn dealer_hand(&self) -> &Hand {
        self.dealer().hand_at(0)
    }
    fn dealer_hand_mut(&mut self) -> &mut Hand {
        self.dealer_mut().hand_at_mut(0)
    }

    fn dealer(&self) -> &Actor {
        self.actors.last().unwrap()
    }

    fn dealer_mut(&mut self) -> &mut Actor {
        self.actors.last_mut().unwrap()
    }

    fn update(&mut self) {
        self.update_state();
        self.print_current_game_state();
    }

    fn print_current_game_state(&self) {
        for actor in self.actors.iter() {
            for (idx, hand) in actor.hands.iter().enumerate() {
                println!("{}:{} {{ {} }}", actor.name, idx + 1, hand.describe());
            }
        }
    }

    fn setup_dealer(&mut self) {
        let dealer = self.dealer_mut();
        dealer.role = ActorRole::Dealer;
        dealer.hand_at_mut(0).card_at_mut(1).hide();
    }

    fn deal_initial_cards(&mut self) {
        for i in 0..self.actors.len() {
            let hand = at!(mut self.actors, i).hand_at_mut(0);
            hand.deal_card(self.deck.draw_card());
            hand.deal_card(self.deck.draw_card());
        }
    }

    fn update_state(&mut self) {
        for actor in self.actors.iter_mut() {
            for hand in actor.hands.iter_mut() {
                hand.update_state();
            }
        }
    }
}

pub fn blackjack_round(number_of_user_players: usize, bet: i32) -> Round {
    let deck = Deck::new_shuffled();
    let mut actors = Vec::with_capacity(number_of_user_players + 1);
    let actor_bets = vec![bet; number_of_user_players]; // dealer is not betting

    for actor_idx in 0..number_of_user_players {
        actors.push(Actor::new(format!("User_{}", actor_idx + 1), Hand::new()))
    }

    actors.push(Actor::new("Dealer".to_string(), Hand::new()));

    Round {
        actors,
        actor_bets,
        deck,
    }
}
