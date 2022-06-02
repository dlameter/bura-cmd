use crate::card::bura;
use crate::game::GameState;
use crate::game::game_state::PlayerState;
use crate::trick::Winner;
use crate::{Card, Player};

pub fn score_trick(state: &mut GameState) {
    if let (Some(trick), Some(trump)) = (&mut state.trick, &state.trump) {
        let winner = get_winning_player(trick.winner(&trump.suit), &mut state.player_state);
        let sum = sum_card_values(trick.take_cards());
        println!("{} won the trick and got {} points!", &winner.name, &sum);
        winner.points += sum;
    }
}

fn get_winning_player(winner: Winner, player_state: &mut PlayerState) -> &mut Player {
    match winner {
        Winner::Lead => player_state.current_player_mut().unwrap(),
        Winner::Follow => {
            player_state.current_player = player_state.next_player_index();
            player_state.current_player_mut().unwrap()
        }
    }
}

fn sum_card_values(cards: Vec<Card>) -> i32 {
    cards
        .iter()
        .map(|card| bura::card_value_to_real_value(&card.value))
        .sum()
}
