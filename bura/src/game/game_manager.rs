use crate::game::GameState;
use crate::Deck;
use crate::Player;
use std::io;
use std::io::Write;

pub struct GameManager<T: Fn(Deck) -> Deck> {
    game_state: GameState,
    shuffler: T,
}

impl<T: Fn(Deck) -> Deck> GameManager<T> {
    pub fn new(game_state: GameState, shuffler: T) -> GameManager<T> {
        GameManager {
            game_state,
            shuffler
        }
    }

    fn ask_for_player() -> Result<Player, String> {
        let mut name = String::new();

        print!("Enter player's name: ");
        io::stdout().flush().expect("Failed to flush stdout while getting player name");
        match io::stdin().read_line(&mut name) {
            Ok(_) => Ok(Player::new(name)),
            Err(_) => Err("Failed to read player name".to_owned()),
        }
    }

    pub fn start(&mut self) -> Option<Player> {
        if self.setup().is_err() {
            return None
        }
        None
    }

    fn setup(&mut self) -> Result<(), ()> {
        match Self::ask_for_player() {
            Ok(player) => self.game_state.players.push(player),
            Err(_) => return Err(())
        }

        match Self::ask_for_player() {
            Ok(player) => self.game_state.players.push(player),
            Err(_) => return Err(())
        }

        Ok(())
    }
}
