pub struct Card {
    pub suit: String,
    pub value: String
}

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

pub struct Player {
    pub name: String,
    pub hand: Hand,
    pub points: i32,
    pub game_points: i32
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            hand: Hand::new(),
            points: 0,
            game_points: 0
        }
    }
}

pub struct GameState {
    pub players: Vec<Player>,
    pub current_player: i32,
    pub deck: Deck,
    pub trick: Trick,
    pub trump: Option<Card>,
}

impl GameState {
    pub fn new(players: Vec<Player>, deck: Deck) -> GameState {
        GameState {
            players,
            current_player: 0,
            deck,
            trick: Trick::new(),
            trump: None
        }
    }
}

pub struct GameManager {
    pub game_state: GameState
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
