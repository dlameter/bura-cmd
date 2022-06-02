use std::fmt::Display;

#[derive(Debug)]
pub struct Card {
    pub suit: String,
    pub value: String,
}

impl Display for Card {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        formatter.write_fmt(format_args!("[{}|{}]", self.suit, self.value))
    }
}

pub mod bura {
    pub fn card_value_to_real_value(card_value: &str) -> i32 {
        match card_value {
            "A" => 11,
            "10" => 10,
            "K" => 4,
            "Q" => 3,
            "J" => 2,
            _ => 0,
        }
    }

    // TODO: Should probably go into the card class somehow as a value mapper
    pub fn card_value_to_order_value(card_value: &str) -> i32 {
        match card_value {
            "A" => 9,
            "10" => 8,
            "K" => 7,
            "Q" => 6,
            "J" => 5,
            "9" => 4,
            "8" => 3,
            "7" => 2,
            "6" => 1,
            _ => 0,
        }
    }
}


pub fn cards_to_string(cards: &[Card]) -> String {
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