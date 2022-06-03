use crate::game::GameState;
use crate::{Card, Hand, Player, Trick, card};
use std::io;
use std::io::Write;
use std::str::FromStr;

pub fn play(state: &mut GameState) {
    let mut cards = Vec::new();
    while cards.is_empty() {
        show_game_info(&state);
        cards = match ask_for_card_selection(state.player_state.current_player().unwrap()).and_then(
            |mut selections| {
                choices_to_cards(
                    &mut state.player_state.current_player_mut().unwrap().hand,
                    &mut selections,
                )
            },
        ) {
            Ok(cards) => cards,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        }
    }

    state.trick.get_or_insert(Trick::default()).play(cards);
}

fn show_game_info(state: &GameState) {
    println!(
        "Trump suit: {}",
        &state
            .trump
            .as_ref()
            .expect("Trump not set when player is playing")
            .suit
    );
    if let Some(lead) = state.trick.as_ref().and_then(|trick| trick.lead.as_ref()) {
        println!("Current lead [{}]", card::cards_to_string(lead));
    };
}

fn ask_for_card_selection(player: &Player) -> Result<Vec<usize>, String> {
    println!("{}'s current hand", &player.name);
    println!("[{}]", card::cards_to_string(&player.hand.cards));

    print!("List the indexes of which cards you'd like to play separated by spaces: ");
    io::stdout()
        .flush()
        .expect("Failed to flush stdout asking player to select cards");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read in player card choices");

    parse_input_choices(input)
}

fn parse_input_choices(input: String) -> Result<Vec<usize>, String> {
    let choices: Vec<Result<usize, _>> = input
        .trim()
        .split_whitespace()
        .map(|string| usize::from_str(string))
        .collect();
    if choices.iter().any(|choice| choice.is_err()) {
        Err("Failed to parse input into integers".to_owned())
    } else {
        Ok(choices.into_iter().map(|choice| choice.unwrap()).collect())
    }
}

fn choices_to_cards(hand: &mut Hand, choices: &mut Vec<usize>) -> Result<Vec<Card>, String> {
    choices.sort_unstable();
    choices.dedup();

    let cards = &mut hand.cards;
    let mut selections = choices.iter_mut().map(|choice| cards.get(*choice));
    if selections.any(|selection| selection.is_none()) {
        return Err("A selection is invalid".to_owned());
    }

    choices.reverse();
    let mut chosen_cards = Vec::new();
    for index in choices {
        chosen_cards.push(cards.remove(*index));
    }

    Ok(chosen_cards)
}
