use crate::game::GameState;

mod play;
mod score_trick;

pub use play::*;
pub use score_trick::*;

const MAX_CARDS: usize = 3;

pub fn draw(state: &mut GameState) {
    if let Some(player) = state.players.get_mut(state.current_player) {
        println!("{} draws until they have {} cards", &player.name, MAX_CARDS);
        while player.hand.cards.len() < MAX_CARDS && state.deck.cards.len() > 0 {
            match state.deck.draw() {
                Some(card) => player.hand.cards.push(card),
                None => break,
            }
        }
    }
}

pub fn change_player(state: &mut GameState) {
    if state.players.len() > 0 {
        state.current_player = (state.current_player + 1) % state.players.len();
        println!(
            "It is now {}'s turn",
            &state.players.get(state.current_player).unwrap().name
        );
    }
}
