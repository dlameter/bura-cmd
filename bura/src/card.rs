use std::fmt::Display;

#[derive(Debug)]
pub struct Card {
    pub suit: String,
    pub value: String
}

impl Display for Card {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        formatter.write_fmt(format_args!("[{}|{}]", self.suit, self.value))
    }
}