use crate::Deck;
use rand::prelude::*;

pub fn rng_shuffle(deck: &mut Deck) {
    deck.cards.shuffle(&mut rand::thread_rng());
}