use std::{io, str::FromStr};

use crate::game::{game_state::PlayerState, GameState};

enum Declaration {
    Yes,
    No,
}

enum ParseDeclarationError {
    EmptyString,
}

impl FromStr for Declaration {
    type Err = ParseDeclarationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().into_iter().next() {
            Some(first_char) => match first_char {
                'y' | 'Y' => Ok(Declaration::Yes),
                _ => Ok(Declaration::No),
            },
            _ => Err(ParseDeclarationError::EmptyString),
        }
    }
}

const WIN_SCORE: i32 = 31;

pub fn declare_win(state: &mut GameState) {
    let mut declaration: Option<Declaration> = None;
    while declaration.is_none() {
        println!(
            "{} do you think you've won and accumulated {} points or more? (yes/no)",
            state.player_state.current_player().unwrap().name,
            WIN_SCORE
        );
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if let Ok(choice) = line.parse() {
            declaration = Some(choice);
        }
    }

    if let Some(Declaration::Yes) = declaration {
        let current_player = state.player_state.current_player().unwrap();
        if current_player.points >= WIN_SCORE {
            println!(
                "{} you've won this round with {} points!",
                current_player.name, current_player.points
            );
            state
                .player_state
                .set_winner(state.player_state.current_player);
            state.player_state.current_player_mut().unwrap().game_points += 1;
            clear_player_scores(&mut state.player_state);
        } else {
            println!(
                "{} you lost this round.  You had {} points.",
                current_player.name, current_player.points
            );
            state
                .player_state
                .set_winner(state.player_state.next_player_index());
            state.player_state.next_player_mut().unwrap().game_points += 1;
            clear_player_scores(&mut state.player_state);
        }
    }
}

fn clear_player_scores(player_state: &mut PlayerState) {
    player_state.current_player_mut().unwrap().points = 0;
    player_state.next_player_mut().unwrap().points = 0;
}
