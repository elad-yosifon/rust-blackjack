use crate::at;
use crate::cards::deck::Deck;
use crate::gameplay::actor::{Actor, ActorRole};
use crate::gameplay::hand::{Hand};

pub struct Round {
    pub deck: Deck,
    pub actors: Vec<Actor>,
    pub actor_bets: Vec<i32>,
    pub actor_cursor: usize,
    pub hand_cursor: usize,
}

#[allow(clippy::derivable_impls)]
impl Default for Round {
    fn default() -> Self {
        Self {
            actors: vec![],
            actor_bets: vec![],
            deck: Deck::default(),
            actor_cursor: 0,
            hand_cursor: 0,
        }
    }
}

impl Round {
    pub fn play(&mut self) {

    }

    pub fn deal_cards(&mut self) {
        self.deal_initial_cards();
        self.setup_dealer();
        self.update();
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

    pub fn update(&mut self) {
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
        deck,
        actors,
        actor_bets,
        hand_cursor: 0,
        actor_cursor: 0,
    }
}
