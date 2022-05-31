use crate::Card;
use crate::Deck;

pub fn standard_deck() -> Deck {
    let mut cards = Vec::new();

    for suit in &["H", "D", "S", "C"] {
        for value in &[
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
        ] {
            cards.push(Card {
                suit: String::from(suit.to_owned()),
                value: String::from(value.to_owned()),
            })
        }
    }

    Deck::new(cards)
}

pub fn bura_deck() -> Deck {
    let mut cards = Vec::new();

    for suit in &["H", "D", "S", "C"] {
        for value in &["A", "6", "7", "8", "9", "10", "J", "Q", "K"] {
            cards.push(Card {
                suit: String::from(suit.to_owned()),
                value: String::from(value.to_owned()),
            })
        }
    }

    Deck::new(cards)
}
