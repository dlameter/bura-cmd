use crate::Card;

pub struct Trick {
    cards: Vec<Card>
}

impl Trick {
    pub fn new() -> Trick {
        Trick {
            cards: Vec::new()
        }
    }

    pub fn add_card(&mut self, card: Card) -> Result<usize, String> {
        if self.cards.len() < 2 {
            self.cards.push(card);
            return Ok(self.cards.len() - 1);
        }
        return Err("Exceeded max cards in a trick".to_owned());
    }

    pub fn get_cards(&self) -> &[Card] {
        return &self.cards
    }

    pub fn leading_suit(&self) -> Option<String> {
        return match self.cards.first() {
            Some(card) => Some(card.suit.clone()),
            None => None
        }
    }
}