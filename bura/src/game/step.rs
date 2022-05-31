use crate::game::GameState;

pub fn draw(state: &mut GameState) {
    if let Some(player) = state.players.get_mut(state.current_player) {
        println!("{} draws until they have 5 cards", &player.name);
        while player.hand.cards.len() < 5 && state.deck.cards.len() > 0 {
            match state.deck.draw() {
                Some(card) => player.hand.cards.push(card),
                None => break
            }
        }
    }
}

pub fn change_player(state: &mut GameState) {
    if state.players.len() > 0 {
        state.current_player = (state.current_player + 1) % state.players.len();
        println!("It is now {}'s turn", &state.players.get(state.current_player).unwrap().name);
    }
}