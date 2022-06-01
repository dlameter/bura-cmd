use crate::game::GameState;
use crate::trick::Winner;

pub fn score_trick(state: &mut GameState) {
    if let (Some(trick), Some(trump)) = (&mut state.trick, &state.trump) {
        let winner = match trick.winner(&trump.suit) {
            Winner::Lead => state.players.get_mut(state.current_player).unwrap(),
            Winner::Follow => {
                let next_index = (state.current_player + 1) % state.players.len();
                state.players.get_mut(next_index).unwrap()
            }
        };

        let won_cards = trick.take_cards();
        let sum: i32 = won_cards
            .iter()
            .map(|card| card_value_to_real_value(&card.value))
            .sum();
        winner.points += sum;
    }
}

fn card_value_to_real_value(card_value: &str) -> i32 {
    match card_value {
        "A" => 11,
        "10" => 10,
        "K" => 4,
        "Q" => 3,
        "J" => 2,
        _ => 0,
    }
}
