use crate::card::Card;
use crate::deck::Deck;
use crate::player::Player;
use crate::trick::Trick;

#[derive(Debug)]
pub struct GameState {
    pub player_state: PlayerState,
    pub deck: Deck,
    pub trick: Option<Trick>,
    pub trump: Option<Card>,
}

impl GameState {
    pub fn new(players: Vec<Player>, deck: Deck) -> GameState {
        GameState {
            player_state: PlayerState::new(players),
            deck,
            trick: None,
            trump: None,
        }
    }
}

#[derive(Debug)]
pub struct PlayerState {
    pub players: Vec<Player>,
    pub current_player: usize,
    pub winner: Option<usize>,
}

impl PlayerState {
    pub fn new(players: Vec<Player>) -> PlayerState {
        PlayerState {
            players,
            current_player: 0,
            winner: None,
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

    pub fn next_player_index(&self) -> usize {
        (self.current_player + 1) % self.players.len()
    }

    pub fn winner(&self) -> Option<&Player> {
        self.winner.and_then(|winner_index| self.players.get(winner_index))
    }

    pub fn set_winner(&mut self, player_index: usize) {
        if self.players.get(player_index).is_some() {
            self.winner = Some(player_index);
        }
    }
}