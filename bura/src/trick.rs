use crate::Card;

#[derive(Debug)]
pub struct Trick {
    pub lead: Vec<Card>,
    pub follow: Vec<Card>
}

impl Trick {
    pub fn new() -> Trick {
        Trick {
            lead: Vec::new(),
            follow: Vec::new()
        }
    }

    pub fn leading_suit(&self) -> Option<String> {
        return match self.lead.first() {
            Some(card) => Some(card.suit.clone()),
            None => None
        }
    }
}