use crate::game::GameState;
use crate::{Card, Hand, Player, Trick};
use std::io;
use std::io::Write;
use std::str::FromStr;

pub fn play(state: &mut GameState) {
    let mut cards = Vec::new();
    while cards.len() == 0 {
        show_game_info(&state);
        cards = match ask_for_card_selection(state.current_player().unwrap()).and_then(
            |mut selections| {
                choices_to_cards(
                    &mut state.current_player_mut().unwrap().hand,
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

    state.trick.get_or_insert(Trick::new()).play(cards);
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
        println!("Current lead [{}]", cards_to_string(lead));
    };
}

fn cards_to_string(cards: &[Card]) -> String {
    let mut iter = cards.iter();
    let mut string = String::new();
    if let Some(first_card) = iter.next() {
        string = format!("{}", first_card);
        for card in iter {
            string += format!(", {}", card).as_ref();
        }
    }
    string
}

fn ask_for_card_selection(player: &Player) -> Result<Vec<usize>, String> {
    println!("{}'s current hand", &player.name);
    display_hand(&player.hand);

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
    if choices.iter().find(|choice| choice.is_err()).is_some() {
        Err("Failed to parse input into integers".to_owned())
    } else {
        Ok(choices.into_iter().map(|choice| choice.unwrap()).collect())
    }
}

fn display_hand(hand: &Hand) {
    for card in &hand.cards {
        print!("{}, ", &card)
    }
}

fn choices_to_cards(hand: &mut Hand, choices: &mut Vec<usize>) -> Result<Vec<Card>, String> {
    choices.sort();
    choices.dedup();

    let cards = &mut hand.cards;
    let mut selections = choices.into_iter().map(|choice| cards.get(*choice));
    if selections.find(|selection| selection.is_none()).is_some() {
        return Err("A selection is invalid".to_owned());
    }

    choices.reverse();
    let mut chosen_cards = Vec::new();
    for index in choices {
        chosen_cards.push(cards.remove(*index));
    }

    return Ok(chosen_cards);
}
