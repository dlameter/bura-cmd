pub mod card;
pub mod deck;
pub mod game;
mod hand;
mod player;
mod trick;

pub use card::Card;
pub use deck::Deck;
pub use hand::Hand;
pub use player::Player;
pub use trick::Trick;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
