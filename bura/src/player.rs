use crate::Hand;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub hand: Hand,
    pub points: i32,
    pub game_points: i32
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            hand: Hand::default(),
            points: 0,
            game_points: 0
        }
    }
}