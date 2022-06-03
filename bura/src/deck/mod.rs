pub mod builder;
pub mod shuffler;

use crate::Card;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(cards: Vec<Card>) -> Deck {
        Deck { cards }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
