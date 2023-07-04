use crate::cards::card::CardSymbol;

pub(crate) fn blackjack_card_value(card_symbol: &CardSymbol) -> i32 {
    match card_symbol {
        CardSymbol::Two => 2,
        CardSymbol::Three => 3,
        CardSymbol::Four => 4,
        CardSymbol::Five => 5,
        CardSymbol::Six => 6,
        CardSymbol::Seven => 7,
        CardSymbol::Eight => 8,
        CardSymbol::Nine => 9,
        CardSymbol::Ten | CardSymbol::Jack | CardSymbol::Queen | CardSymbol::King => 10,
        CardSymbol::Ace => 11,
        CardSymbol::Joker => 0,
    }
}

pub(crate) enum UserAction {
    Hit,
    Stay,
    Split,
}
