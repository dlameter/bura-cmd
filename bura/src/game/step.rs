use std::io;

use crate::game::GameState;

mod play;
mod score_trick;
mod declare_win;

pub use play::*;
pub use score_trick::*;
pub use declare_win::*;

const MAX_CARDS: usize = 3;

pub fn draw(state: &mut GameState) {
    println!("{} draws until they have {} cards", &state.player_state.current_player().unwrap().name, MAX_CARDS);
    while state.player_state.current_player().unwrap().hand.cards.len() < MAX_CARDS && state.deck.cards.len() > 0 {
        match state.deck.draw() {
            Some(card) => state.player_state.current_player_mut().unwrap().hand.cards.push(card),
            None => break,
        }
    }
}

pub fn change_player(state: &mut GameState) {
    if state.player_state.players.len() > 0 {
        state.player_state.current_player = state.player_state.next_player_index();
        println!(
            "It is now {}'s turn",
            &state.player_state.players.get(state.player_state.current_player).unwrap().name
        );
    }
}

pub fn clear_screen(_state: &mut GameState) {
    print!("{esc}c", esc = 27 as char);
}

pub fn confirm_player(state: &mut GameState) {
    println!("Confirm {} player", state.player_state.current_player().unwrap().name);
    io::stdin().read_line(&mut String::new()).unwrap();
}