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

    pub fn current_player(&self) -> Option<&Player> {
        self.players.get(self.current_player)
    }

    pub fn current_player_mut(&mut self) -> Option<&mut Player> {
        self.players.get_mut(self.current_player)
    }

    pub fn next_player(&self) -> Option<&Player> {
        self.players.get(self.next_player_index())
    }

    pub fn next_player_mut(&mut self) -> Option<&mut Player> {
        let index = self.next_player_index();
        self.players.get_mut(index)
    }

    fn next_player_index(&self) -> usize {
        (self.current_player + 1) % self.players.len()
    }
}
