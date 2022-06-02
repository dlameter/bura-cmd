use crate::game::GameState;
use crate::Deck;
use crate::Player;
use std::io;
use std::io::Write;

type GameStep = fn(&mut GameState);

pub struct GameManager<T: Fn(&mut Deck)> {
    game_state: GameState,
    shuffler: T,
    game_steps: Vec<GameStep>
}

impl<T: Fn(&mut Deck)> GameManager<T> {
    pub fn new(game_state: GameState, shuffler: T) -> GameManager<T> {
        GameManager {
            game_state,
            shuffler,
            game_steps: Vec::new()
        }
    }

    pub fn add_step(&mut self, step: GameStep) {
        self.game_steps.push(step);
    }

    fn ask_for_player() -> Result<Player, String> {
        let mut name = String::new();

        print!("Enter player's name: ");
        io::stdout().flush().expect("Failed to flush stdout while getting player name");
        match io::stdin().read_line(&mut name) {
            Ok(_) => Ok(Player::new(name.trim().to_owned())),
            Err(_) => Err("Failed to read player name".to_owned()),
        }
    }

    pub fn start(&mut self) -> Option<Player> {
        if self.setup().is_err() {
            return None
        }

        while self.game_state.player_state.winner().is_none() {
            for step in &self.game_steps {
                step(&mut self.game_state);
                if self.game_state.player_state.winner().is_some() {
                    break;
                }
            }
        }

        println!("Players: {:#?}", self.game_state.player_state.players);
        None
    }

    fn setup(&mut self) -> Result<(), ()> {
        match Self::ask_for_player() {
            Ok(player) => self.game_state.player_state.players.push(player),
            Err(_) => return Err(())
        }

        match Self::ask_for_player() {
            Ok(player) => self.game_state.player_state.players.push(player),
            Err(_) => return Err(())
        }

        (self.shuffler)(&mut self.game_state.deck);

        self.game_state.trump = self.game_state.deck.draw();

        Ok(())
    }
}
