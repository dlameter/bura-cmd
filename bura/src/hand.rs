use crate::Card;

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>
}

impl Default for Hand {
    fn default() -> Self {
        Hand {
            cards: Vec::new()
        }
    }
}