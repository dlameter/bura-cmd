use crate::Card;

#[derive(Debug)]
pub struct Trick {
    pub lead: Option<Vec<Card>>,
    pub follow: Option<Vec<Card>>,
}

impl Trick {
    pub fn new() -> Trick {
        Trick {
            lead: None,
            follow: None,
        }
    }

    pub fn leading_suit(&self) -> Option<String> {
        return match self.lead.as_ref().and_then(|lead| lead.first()) {
            Some(card) => Some(card.suit.clone()),
            None => None,
        };
    }

    pub fn play(&mut self, cards: Vec<Card>) {
        if self.lead.is_none() {
            self.lead = Some(cards);
        } else if self.follow.is_none() {
            self.follow = Some(cards);
        } else {
            panic!("tried to play a hand on a full trick!");
        }
    }

    pub fn take_cards(&mut self) -> Vec<Card> {
        let mut cards = Vec::new();

        if let Some(mut lead_cards) = self.lead.take() {
            cards.append(&mut lead_cards);
            self.lead = None;
        }

        if let Some(mut follow_cards) = self.follow.take() {
            cards.append(&mut follow_cards);
            self.follow = None;
        }

        cards
    }
}
