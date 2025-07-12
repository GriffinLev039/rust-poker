use core::fmt;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Value {
    pub fn numeric_value(&self) -> u8 {
        match self {
            Value::Ace => 14,
            Value::King => 13,
            Value::Queen => 12,
            Value::Jack => 11,
            Value::Ten => 10,
            Value::Nine => 9,
            Value::Eight => 8,
            Value::Seven => 7,
            Value::Six => 6,
            Value::Five => 5,
            Value::Four => 4,
            Value::Three => 3,
            Value::Two => 2,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub struct PlayingCard {
    suit: Suit,
    value: Value,
}

impl PlayingCard {
    pub fn new(s: Suit, v: Value) -> PlayingCard {
        PlayingCard { suit: s, value: v }
    }
    //Standard getter.
    pub fn get_value(&self) -> Value {
        self.value
    }
    //Standard getter.
    pub fn get_suit(&self) -> Suit {
        self.suit
    }
    //Gets numeric value of card.
    pub fn numeric_value(&self) -> u8 {
        self.value.numeric_value()
    }
}

//Determines display syntax for card
impl fmt::Display for PlayingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.get_suit(), self.get_value())
    }
}

//Determines display syntax for card's Suit.
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Heart => write!(f, "♥"),
            Suit::Diamond => write!(f, "♦"),
            Suit::Club => write!(f, "♣"),
            Suit::Spade => write!(f, "♠"),
        }
    }
}

//Determines display syntax for card's value.
impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Ace => write!(f, "A"),
            Value::King => write!(f, "K"),
            Value::Queen => write!(f, "Q"),
            Value::Jack => write!(f, "J"),
            Value::Ten => write!(f, "10"),
            Value::Nine => write!(f, "9"),
            Value::Eight => write!(f, "8"),
            Value::Seven => write!(f, "7"),
            Value::Six => write!(f, "6"),
            Value::Five => write!(f, "5"),
            Value::Four => write!(f, "4"),
            Value::Three => write!(f, "3"),
            Value::Two => write!(f, "2"),
        }
    }
}

//Unsure if this is necessary or if I can just derive this. Will test.
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.numeric_value().cmp(&other.numeric_value()))
    }
}

//Order of value, used for order of cards as well.
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value: u8 = self.numeric_value();
        let other_value: u8 = other.numeric_value();
        self_value.cmp(&other_value)
    }
}

impl PartialOrd for PlayingCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.numeric_value().cmp(&other.numeric_value()))
    }
}

//Unsure if I can derive this or not, will test.
impl Ord for PlayingCard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.numeric_value().cmp(&other.numeric_value())
    }
}

//PRIVATE TESTS ONLY!! ALL PUBLIC TESTING SHOULD BE DONE IN MAIN!
#[cfg(test)]
mod tests {}
