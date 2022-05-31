use crate::card::Card;
use crate::deck::Deck;
use crate::player::Player;
use crate::trick::Trick;

#[derive(Debug)]
pub struct GameState {
    pub players: Vec<Player>,
    pub current_player: usize,
    pub deck: Deck,
    pub trick: Option<Trick>,
    pub trump: Option<Card>,
}

impl GameState {
    pub fn new(players: Vec<Player>, deck: Deck) -> GameState {
        GameState {
            players,
            current_player: 0,
            deck,
            trick: None,
            trump: None,
        }
    }
}
