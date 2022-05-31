use crate::Card;
use crate::Deck;
use crate::game::GameState;
use crate::Player;
use std::io;

pub struct GameManager {
    pub game_state: GameState,
}

impl GameManager {
    pub fn ask_for_player() -> Result<Player, String> {
        let mut name = String::new();
        match io::stdin().read_line(&mut name) {
            Ok(_) => Ok(Player::new(name)),
            Err(_) => Err("Failed to read player name".to_owned()),
        }
    }

    fn build_deck() -> Deck {
        let mut cards = Vec::new();

        for suit in &["H", "D", "S", "C"] {
            for value in &[
                "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
            ] {
                cards.push(Card {
                    suit: String::from(suit.to_owned()),
                    value: String::from(value.to_owned()),
                })
            }
        }

        Deck::new(cards)
    }

    pub fn start(&mut self) -> Option<Player> {
        GameManager::build_deck();
        None
    }
}
