use crate::game::GameState;
use crate::trick::Winner;
use crate::{Card, Player};

pub fn score_trick(state: &mut GameState) {
    if let (Some(trick), Some(trump)) = (&mut state.trick, &state.trump) {
        let sum = sum_card_values(trick.take_cards());
        let winner = get_winning_player(trick.winner(&trump.suit), state);
        winner.points += sum;
    }
}

fn get_winning_player(winner: Winner, state: &mut GameState) -> &mut Player {
    match winner {
        Winner::Lead => state.players.get_mut(state.current_player).unwrap(),
        Winner::Follow => {
            let next_index = (state.current_player + 1) % state.players.len();
            state.players.get_mut(next_index).unwrap()
        }
    }
}

fn sum_card_values(cards: Vec<Card>) -> i32 {
    cards
        .iter()
        .map(|card| card_value_to_real_value(&card.value))
        .sum()
}

// TODO: Should be associated to Card in some way, not in here
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
