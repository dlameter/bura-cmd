use crate::Deck;

pub fn build_swap_shuffler(swaps: i32) -> impl Fn(Deck) -> Deck {
    move |deck: Deck| {
        let mut deck = deck;

        for _ in 0..swaps {
            deck = swap_shuffle(deck);
        }

        deck
    }
}

pub fn swap_shuffle(deck: Deck) -> Deck {
    deck.cards.len();


    deck
}

fn swap_cards(deck: Deck, index1: usize, index2: usize) -> Deck {
    deck
}