use crate::Card;

pub struct Hand {
    pub cards: Vec<Card>
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: Vec::new()
        }
    }
}