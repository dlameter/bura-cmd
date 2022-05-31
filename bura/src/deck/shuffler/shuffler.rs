use crate::Deck;
use rand::prelude::*;

pub fn shuffle(mut deck: Deck) -> Deck {
    deck.cards.shuffle(&mut rand::thread_rng());
    deck
}