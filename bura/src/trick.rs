use crate::{Card, card::bura};

#[derive(PartialEq)]
pub enum Winner {
    Lead,
    Follow,
}

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

    pub fn winner(&self, trump_suit: &str) -> Winner {
        if self.lead.is_some() && self.follow.is_some() {
            let lead = self.lead.as_ref().unwrap();
            let follow = self.follow.as_ref().unwrap();

            lead.iter()
                .zip(follow.iter())
                .map(|(lead_card, follow_card)| {
                    Self::score_stack(trump_suit, lead_card, follow_card)
                })
                .fold(Winner::Follow, |win1, win2| {
                    if win1 == win2 {
                        win1
                    } else {
                        Winner::Lead
                    }
                })
        } else {
            Winner::Lead
        }
    }

    fn score_stack(trump_suit: &str, lead_card: &Card, follow_card: &Card) -> Winner {
        if lead_card.suit == follow_card.suit {
            if bura::card_value_to_order_value(&lead_card.value)
                >= bura::card_value_to_order_value(&follow_card.value)
            {
                Winner::Lead
            } else {
                Winner::Follow
            }
        } else {
            if follow_card.suit == trump_suit {
                Winner::Follow
            } else {
                Winner::Lead
            }
        }
    }
}
