use crate::game::GameState;
use crate::{Card, Hand, Player, Trick};
use std::io;
use std::str::FromStr;

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

pub fn play(state: &mut GameState) {
    if let Some(player) = state.players.get_mut(state.current_player) {
        let cards = Vec::new();
        while cards.len() == 0 {
            let selections = ask_for_card_selection(player);
        }

        state.trick.get_or_insert(Trick::new()).play(cards);
    }
}

fn ask_for_card_selection(player: &Player) -> Result<Vec<Card>, String> {
    println!("{}'s current hand", &player.name);
    display_hand(&player.hand);

    print!("List the indexes of which cards you'd like to play separated by spaces: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read in player card choices");

    let choices = parse_input_choices(input);
    choices_to_cards(&player.hand, &choices.unwrap())
}

fn parse_input_choices(input: String) -> Result<Vec<usize>, String> {
    let mut choices = input
        .split_whitespace()
        .map(|string| usize::from_str(string));
    if choices.find(|choice| choice.is_err()).is_some() {
        Err("Failed to parse input into integers".to_owned())
    } else {
        Ok(choices.map(|choice| choice.unwrap()).collect())
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

    let cards = &hand.cards;
    let mut selections = choices.into_iter().map(|choice| cards.get(*choice));
    if selections.find(|selection| selection.is_none()).is_some() {
        return Err("A selection is invalid".to_owned());
    }

    choices.reverse();
    let chosen_cards = Vec::new();
    for index in choices {
        chosen_cards.push(cards.remove(*index));
    }

    return Ok(chosen_cards);
}