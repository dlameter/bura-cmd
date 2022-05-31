use crate::Card;

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn new(cards: Vec<Card>) -> Deck {
        Deck {
            cards
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        return self.cards.pop()
    }
}