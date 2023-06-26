use crate::cards::Deck;

mod cards;
mod players;

struct Hand {}
struct PlayerState {}
struct Game {}
struct Round {}

fn main() {
    let mut deck = Deck::new();
    println!("new deck with {} cards in it", deck.cards.len());
    deck.shuffle();
    for card in deck.cards.iter() {
        println!("Card: color={}, suit={}, value={}",
                 card.suit.color().to_str(),
                 card.suit.symbol(),
                 card.value.to_str()
        );
    }
}
