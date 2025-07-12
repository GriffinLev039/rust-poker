use core::fmt;
use rand::rng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;
use std::collections::HashMap;

pub mod card;
pub mod hand_type;
use card::{PlayingCard, Suit, Value};
use hand_type::HandType;

// Could be simplified significantly by embedding various parts into the enum
// Learned about that after writing this though, so I'll have to refactor eventually.

// CONSTANTS
const DEFAULT_HAND_SIZE: usize = 5;
const ACE_VALUE: u8 = 14;

///Hand structure
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Hand {
    hand: Vec<card::PlayingCard>,
    max_size: usize,
    hand_type: HandType,
}

impl Hand {
    // ---------------------------
    // Constructors
    // ---------------------------

    //Default constructor.
    // Likely won't be used, probs will remove in favor of 'from' constructor.
    // Creates hand at default poker size (5 - magic number issue??)
    pub fn default() -> Hand {
        let mut hand = Hand {
            hand: Vec::new(),
            max_size: DEFAULT_HAND_SIZE,
            hand_type: HandType::None,
        };
        hand.hand_type = HandType::None;
        hand
    }
    //Constructs Hand obj from a given vector
    pub fn from(v: Vec<PlayingCard>) -> Hand {
        let mut hand = Hand {
            hand: v.clone(),
            max_size: v.len(),
            hand_type: HandType::None,
        };
        hand.hand_type = hand.check_hand_type();
        hand.hand.sort();
        hand.hand.reverse();
        hand
    }

    //Allows creation of hands of custom sizes.
    fn new(m_size: usize) -> Hand {
        Hand {
            hand: Vec::new(),
            max_size: m_size,
            hand_type: HandType::None,
        }
    }

    pub fn generate_deck() -> Hand {
        let mut hand: Hand = Hand::new(52);
        hand.hand_type = HandType::Deck;
        for &suit in &[Suit::Club, Suit::Spade, Suit::Heart, Suit::Diamond] {
            for &value in &[
                Value::Ace,
                Value::King,
                Value::Queen,
                Value::Jack,
                Value::Ten,
                Value::Nine,
                Value::Eight,
                Value::Seven,
                Value::Six,
                Value::Five,
                Value::Four,
                Value::Three,
                Value::Two,
            ] {
                let _ = hand.draw(PlayingCard::new(suit, value));
            }
        }
        hand.hand.shuffle(&mut rng());
        hand
    }

    // -------------------------
    // Card Operations
    // -------------------------

    //Draws a card, essentially a single-purpose setter.
    pub fn draw(&mut self, card: PlayingCard) -> Result<PlayingCard, &str> {
        if self.hand.len() < self.max_size.into() {
            self.hand.push(card);
            if self.hand.len() == self.max_size {
                self.hand_type = self.check_hand_type();
            }
            Ok(card)
        } else {
            Err("Hand is full")
        }
    }
    //Just a getter.
    pub fn get_hand(&self) -> &Vec<PlayingCard> {
        &self.hand
    }

    //Peeks at the top card without discarding
    pub fn peek(&mut self) -> Option<PlayingCard> {
        if self.hand.len() != 0 {
            return Some(self.hand[0]);
        }
        None
    }

    pub fn deal(&mut self, other: &mut Hand) {
        if other.hand.len() < other.max_size && self.hand.len() > 0 {
            other
                .draw(self.discard(0).unwrap())
                .expect("Already checks possible fail conditions.");
        }
    }

    //Removes a card at a given index. Due to the nature of Vec cards seem to propogate to lowest possible indexes.
    //Need to test that tho.
    pub fn discard(&mut self, index: usize) -> Option<PlayingCard> {
        if index < self.hand.len() {
            Some(self.hand.remove(index))
        } else {
            None
        }
    }

    pub fn group_discard(&mut self, mut indexes: Vec<usize>) {
        indexes.sort();
        indexes.reverse();
        for num in indexes {
            self.hand.remove(num);
        }
    }

    // -------------------------------
    // HAND COMPARISON / EVALUATION
    // -------------------------------

    /*
    * Used to compare hands when one hand doesn't supercede another by ID alone
    *   - Royal Flushes will always tie.
        - Any straight will always be compared by highest card.
        - Flushes/High cards will always be compared by iterating down till a difference is found.
        - Four/Three kind will always have a set card in specific spot due to sorting - if
          a card is pulled from the middle, then it should match up no matter what when cards are sorted by value.
                        - 0 0 0 x x
                        - 0 0 0 0 x
        - Two Pair, Pair, and Full House are explaine within match function.
     */
    fn compare_equal_hands(&self, other: Hand) -> Ordering {
        match self.hand_type {
            HandType::RoyalFlush => {
                //ALWAYS A DRAW BY NATURE
                Ordering::Equal
            }

            HandType::StraightFlush => {
                //Compare top card in hand!
                self.get_hand()[0].cmp(&other.get_hand()[0])
            }

            HandType::FourKind => {
                //Compare middle card in hand!
                self.get_hand()[2].cmp(&other.get_hand()[2])
            }

            HandType::FullHouse => {
                //Compare Three of a Kinds, then compare two of a kinds!
                // Self three card
                // Self two card
                // Other three card
                // Other two card
                let self_three_card: PlayingCard;
                let self_two_card: PlayingCard;
                let other_three_card: PlayingCard;
                let other_two_card: PlayingCard;

                if self.get_hand()[0] == self.get_hand()[2] {
                    self_three_card = self.get_hand()[0];
                    self_two_card = self.get_hand()[4];
                } else {
                    self_three_card = self.get_hand()[4];
                    self_two_card = self.get_hand()[0];
                }
                if other.get_hand()[0] == other.get_hand()[2] {
                    other_three_card = self.get_hand()[0];
                    other_two_card = self.get_hand()[4];
                } else {
                    other_three_card = self.get_hand()[4];
                    other_two_card = self.get_hand()[0];
                }
                if self_three_card.cmp(&other_three_card) == Ordering::Equal {
                    self_two_card.cmp(&other_two_card)
                } else {
                    self_three_card.cmp(&other_three_card)
                }
            }

            HandType::Flush => {
                //Compare top card in hand!
                for i in 0..self.get_hand().len() {
                    if self.get_hand()[i].cmp(&other.get_hand()[i]) != Ordering::Equal {
                        return self.get_hand()[i].cmp(&other.get_hand()[i]);
                    }
                }
                Ordering::Equal
            }

            HandType::Straight => {
                // Compare top card in hand!
                self.get_hand()[0].cmp(&other.get_hand()[0])
            }

            HandType::ThreeKind => {
                //Compare middle card
                self.get_hand()[2].cmp(&other.get_hand()[2])
            }

            HandType::TwoPair => {
                //Compare each hands highest hand.
                //If those match, compare other hand.
                //If those match, compare remaining non-pair card.
                // Since a two pair can be broken down into three parts: the higher pair, the lower pair, and the single kicker,
                // and since the hand is ordered, pairs auto sort themselves, meaning there is three possible orders for a two pair hand:
                //              - AA BB C
                //              - AA C BB
                //              - C AA BB
                // Where AA is the first pair, BB is the second pair, and C is a kicker.
                let self_higher_pair: PlayingCard;
                let self_lower_pair: PlayingCard;
                let self_kicker: PlayingCard;
                let other_higher_pair: PlayingCard;
                let other_lower_pair: PlayingCard;
                let other_kicker: PlayingCard;
                if self.get_hand()[0] == self.get_hand()[1] {
                    self_higher_pair = self.get_hand()[0];
                    if self.get_hand()[4] == self.get_hand()[3] {
                        self_lower_pair = self.get_hand()[4];
                        self_kicker = self.get_hand()[2];
                    } else {
                        self_lower_pair = self.get_hand()[2];
                        self_kicker = self.get_hand()[4];
                    }
                } else {
                    self_kicker = self.get_hand()[0];
                    self_higher_pair = self.get_hand()[1];
                    self_lower_pair = self.get_hand()[3];
                }

                if other.get_hand()[0] == other.get_hand()[1] {
                    other_higher_pair = other.get_hand()[0];
                    if other.get_hand()[4] == other.get_hand()[3] {
                        other_lower_pair = other.get_hand()[4];
                        other_kicker = other.get_hand()[2];
                    } else {
                        other_lower_pair = other.get_hand()[2];
                        other_kicker = other.get_hand()[4];
                    }
                } else {
                    other_kicker = other.get_hand()[0];
                    other_higher_pair = other.get_hand()[1];
                    other_lower_pair = other.get_hand()[3];
                }

                if self_higher_pair.cmp(&other_higher_pair) == Ordering::Equal {
                    if self_lower_pair.cmp(&other_lower_pair) == Ordering::Equal {
                        self_kicker.cmp(&other_kicker)
                    } else {
                        self_lower_pair.cmp(&other_lower_pair)
                    }
                } else {
                    self_higher_pair.cmp(&other_higher_pair)
                }
            }

            HandType::Pair => {
                // Pairs will incorrectly report EQUAL when both are equal but a kicker exists - need to update.
                // Need better practice than assigning a default value here.
                let mut self_pair: PlayingCard = PlayingCard::new(Suit::Club, Value::Eight);
                let mut other_pair: PlayingCard = PlayingCard::new(Suit::Club, Value::Eight);
                for j in 0..(self.get_hand().len() - 1) {
                    if self.get_hand()[j].cmp(&self.get_hand()[j + 1]) == Ordering::Equal {
                        self_pair = self.get_hand()[j];
                    }
                    if other.get_hand()[j].cmp(&other.get_hand()[j + 1]) == Ordering::Equal {
                        other_pair = other.get_hand()[j];
                    }
                }
                self_pair.cmp(&other_pair)
            }

            HandType::HighCard => {
                // Compare highest card. If equal, go in sequence till theres a difference.
                for i in 0..self.get_hand().len() {
                    if self.get_hand()[i].cmp(&other.get_hand()[i]) != Ordering::Equal {
                        return self.get_hand()[i].cmp(&other.get_hand()[i]);
                    }
                }
                Ordering::Equal
            }

            _ => Ordering::Equal,
        }
    }

    // Checks if all cards posses the same 'suit' attribute.
    fn is_flush(&self) -> bool {
        let suit: Suit = self.get_hand()[0].get_suit();
        for c in self.get_hand() {
            if c.get_suit() != suit {
                return false;
            }
        }
        true
    }
    // Checks if all card values are within one of the next.
    // Fails on straights using both a 2 and an ace - need to
    // fix that. Otherwise fine.
    fn is_straight(&mut self) -> bool {
        print!(
            "Sorted hand: {:?}",
            self.hand
                .iter()
                .map(|c| c.numeric_value())
                .collect::<Vec<_>>()
        );
        if self.get_hand()[0].numeric_value() == 14 && self.get_hand()[1].numeric_value() == 5 {
            for i in 1..self.hand.len() - 1 {
                if self.get_hand()[i].numeric_value() - self.get_hand()[i + 1].numeric_value() != 1
                {
                    println!(" Result: False");
                    return false;
                }
            }
            return true;
        }
        for i in 0..self.hand.len() - 1 {
            if self.get_hand()[i].numeric_value() - self.get_hand()[i + 1].numeric_value() != 1 {
                println!(" Result: False");
                return false;
            }
        }
        println!(" Result: True");

        true
    }
    //Checks all possible set/pair combinations.
    //Pseudo-Code:
    /* if a key's value is 4
        then return FourKind
    else if a key is 3 or 2
        then if a different key is 3
            return fullhouse
        else if a different key is 2 and the first key is 3
            return fullhouse
        else if a different key is 2
            return twopair
    else if a key is 2
        return pair
    else
        return highcard

     */
    fn check_pairs(&self) -> HandType {
        let mut hmap = HashMap::<u8, u8>::new();
        for c in self.get_hand() {
            let num = c.numeric_value();
            if hmap.contains_key(&num) {
                hmap.insert(num, hmap.get(&num).unwrap() + 1);
            } else {
                hmap.insert(num, 1);
            }
        }
        for (key, val) in &hmap {
            if *val == 4 {
                return HandType::FourKind;
            } else if *val == 3 || *val == 2 {
                for (sub_key, sub_val) in &hmap {
                    if *sub_key != *key {
                        if *sub_val == 3 {
                            return HandType::FullHouse;
                        }
                        if *sub_val == 2 {
                            if *val == 2 {
                                return HandType::TwoPair;
                            } else {
                                return HandType::FullHouse;
                            }
                        }
                    }
                }
                if *val == 3 {
                    return HandType::ThreeKind;
                } else {
                    return HandType::Pair;
                }
            }
        }
        HandType::HighCard
    }

    fn check_hand_type(&mut self) -> HandType {
        if self.hand_type == HandType::Deck {
            return HandType::Deck;
        }
        self.hand.sort();
        self.hand.reverse();

        if self.is_flush() && self.is_straight() {
            if self.hand[0].numeric_value() == ACE_VALUE {
                return HandType::RoyalFlush;
            } else {
                return HandType::StraightFlush;
            }
        } else if self.is_flush() {
            return HandType::Flush;
        } else if self.is_straight() {
            return HandType::Straight;
        } else {
            return self.check_pairs();
        }
    }

    fn hand_value(&self) -> u8 {
        match self.hand_type {
            HandType::RoyalFlush => 10,
            HandType::StraightFlush => 9,
            HandType::FourKind => 8,
            HandType::FullHouse => 7,
            HandType::Flush => 6,
            HandType::Straight => 5,
            HandType::ThreeKind => 4,
            HandType::TwoPair => 3,
            HandType::Pair => 2,
            HandType::HighCard => 1,
            HandType::None => 0,
            HandType::Deck => 0,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_value().cmp(&other.hand_value()) == Ordering::Equal {
            Hand::compare_equal_hands(self, other.clone())
        } else {
            self.hand_value().cmp(&other.hand_value())
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in self.get_hand() {
            write!(f, "{} , ", card)?;
        }
        Ok(())
    }
}

//PRIVATE TESTS ONLY!! ALL PUBLIC TESTING SHOULD BE DONE IN MAIN!
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hand_match_test() {
        // Test Royal Flush
        let royal_flush_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::King),
            PlayingCard::new(Suit::Club, Value::Queen),
            PlayingCard::new(Suit::Club, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);
        assert_eq!(royal_flush_hand.hand[0].get_value(), Value::Ace);
        assert_eq!(royal_flush_hand.hand[0].numeric_value(), 14);
        assert_eq!(royal_flush_hand.hand_type, HandType::RoyalFlush);
        // Test Straight Flush
        let straight_flush_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::King),
            PlayingCard::new(Suit::Club, Value::Queen),
            PlayingCard::new(Suit::Club, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Ten),
            PlayingCard::new(Suit::Club, Value::Nine),
        ]);
        assert_eq!(straight_flush_hand.hand_type, HandType::StraightFlush);
        // Test Four Kind
        let four_of_a_kind_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Diamond, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);
        assert_eq!(four_of_a_kind_hand.hand_type, HandType::FourKind);
        //Full House
        let full_house_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Diamond, Value::Ten),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);
        assert_eq!(full_house_hand.hand_type, HandType::FullHouse);

        // Test Straight
        let straight_hand = Hand::from(vec![
            PlayingCard::new(Suit::Diamond, Value::Ace),
            PlayingCard::new(Suit::Club, Value::King),
            PlayingCard::new(Suit::Club, Value::Queen),
            PlayingCard::new(Suit::Club, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);
        assert_eq!(straight_hand.hand_type, HandType::Straight);

        //Test 5-Ace Straight
        let straight_two_hand: Hand = Hand::from(vec![
            PlayingCard::new(Suit::Diamond, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Five),
            PlayingCard::new(Suit::Club, Value::Four),
            PlayingCard::new(Suit::Club, Value::Three),
            PlayingCard::new(Suit::Club, Value::Two),
        ]);
        assert_eq!(straight_two_hand.hand_type, HandType::Straight);

        // Test FLush
        let flush_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Queen),
            PlayingCard::new(Suit::Club, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);
        assert_eq!(flush_hand.hand_type, HandType::Flush);
        // Test Three Kind
        let three_kind_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Diamond, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Diamond, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);
        assert_eq!(three_kind_hand.hand_type, HandType::ThreeKind);
        // Test Two Pair
        let two_pair_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Diamond, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Jack),
            PlayingCard::new(Suit::Diamond, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);
        assert_eq!(two_pair_hand.hand_type, HandType::TwoPair);
        // Test Pair
        let pair_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ten),
            PlayingCard::new(Suit::Diamond, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Jack),
            PlayingCard::new(Suit::Diamond, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Nine),
        ]);
        assert_eq!(pair_hand.hand_type, HandType::Pair);
        // Test High Card
        let high_card_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Diamond, Value::King),
            PlayingCard::new(Suit::Club, Value::Three),
            PlayingCard::new(Suit::Diamond, Value::Two),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);
        assert_eq!(high_card_hand.hand_type, HandType::HighCard);
    }

    #[test]
    fn comparison_test() {
        //Compare Royal Flush (Always equal afaik)
        let royal_flush_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::King),
            PlayingCard::new(Suit::Club, Value::Queen),
            PlayingCard::new(Suit::Club, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);

        let other_royal_flush_hand = Hand::from(vec![
            PlayingCard::new(Suit::Diamond, Value::Ace),
            PlayingCard::new(Suit::Diamond, Value::King),
            PlayingCard::new(Suit::Diamond, Value::Queen),
            PlayingCard::new(Suit::Diamond, Value::Jack),
            PlayingCard::new(Suit::Diamond, Value::Ten),
        ]);

        assert_eq!(
            royal_flush_hand.cmp(&other_royal_flush_hand),
            Ordering::Equal
        );
        //Compare straight flush
        let straight_flush_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::King),
            PlayingCard::new(Suit::Club, Value::Queen),
            PlayingCard::new(Suit::Club, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Ten),
            PlayingCard::new(Suit::Club, Value::Nine),
        ]);

        let other_straight_flush_hand = Hand::from(vec![
            PlayingCard::new(Suit::Diamond, Value::Ten),
            PlayingCard::new(Suit::Diamond, Value::Nine),
            PlayingCard::new(Suit::Diamond, Value::Eight),
            PlayingCard::new(Suit::Diamond, Value::Seven),
            PlayingCard::new(Suit::Diamond, Value::Six),
        ]);
        assert_eq!(straight_flush_hand.hand_type, HandType::StraightFlush);
        assert_eq!(other_straight_flush_hand.hand_type, HandType::StraightFlush);

        assert_eq!(
            straight_flush_hand.cmp(&other_straight_flush_hand),
            Ordering::Greater
        );
        //Compare four kind
        let four_kind_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Diamond, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);

        let other_four_kind_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ten),
            PlayingCard::new(Suit::Club, Value::Ten),
            PlayingCard::new(Suit::Diamond, Value::Ten),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);
        assert_eq!(four_kind_hand.cmp(&other_four_kind_hand), Ordering::Greater);
        //Compare full house
        let full_house_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Diamond, Value::Ten),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);

        let other_full_house_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Nine),
            PlayingCard::new(Suit::Club, Value::Nine),
            PlayingCard::new(Suit::Club, Value::Nine),
            PlayingCard::new(Suit::Diamond, Value::Ten),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);
        assert_eq!(
            full_house_hand.cmp(&other_full_house_hand),
            Ordering::Greater
        );
        //Compare flush
        let flush_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::King),
            PlayingCard::new(Suit::Club, Value::Queen),
            PlayingCard::new(Suit::Club, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Nine),
        ]);

        let other_flush_hand = Hand::from(vec![
            PlayingCard::new(Suit::Diamond, Value::Ten),
            PlayingCard::new(Suit::Diamond, Value::Nine),
            PlayingCard::new(Suit::Diamond, Value::Eight),
            PlayingCard::new(Suit::Diamond, Value::Seven),
            PlayingCard::new(Suit::Diamond, Value::Five),
        ]);
        assert_eq!(flush_hand.cmp(&other_flush_hand), Ordering::Greater);

        //Compare straight
        let straight_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Spade, Value::King),
            PlayingCard::new(Suit::Club, Value::Queen),
            PlayingCard::new(Suit::Club, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);

        let other_straight_hand = Hand::from(vec![
            PlayingCard::new(Suit::Diamond, Value::Ten),
            PlayingCard::new(Suit::Diamond, Value::Nine),
            PlayingCard::new(Suit::Club, Value::Eight),
            PlayingCard::new(Suit::Diamond, Value::Seven),
            PlayingCard::new(Suit::Diamond, Value::Six),
        ]);
        assert_eq!(straight_hand.cmp(&other_straight_hand), Ordering::Greater);

        //Compare three kind
        let three_kind_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Diamond, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);

        let other_three_kind_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ten),
            PlayingCard::new(Suit::Club, Value::Ten),
            PlayingCard::new(Suit::Diamond, Value::Ten),
            PlayingCard::new(Suit::Club, Value::Nine),
        ]);
        assert_eq!(
            three_kind_hand.cmp(&other_three_kind_hand),
            Ordering::Greater
        );

        //Compare pair
        let pair_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Queen),
            PlayingCard::new(Suit::Diamond, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Ten),
        ]);

        let other_pair_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Ten),
            PlayingCard::new(Suit::Diamond, Value::Ten),
            PlayingCard::new(Suit::Club, Value::Nine),
        ]);
        assert_eq!(pair_hand.cmp(&other_pair_hand), Ordering::Greater);
        //Compare high card
        let high_card_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::Ace),
            PlayingCard::new(Suit::Club, Value::King),
            PlayingCard::new(Suit::Club, Value::Queen),
            PlayingCard::new(Suit::Diamond, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Nine),
        ]);

        let other_high_card_hand = Hand::from(vec![
            PlayingCard::new(Suit::Club, Value::King),
            PlayingCard::new(Suit::Club, Value::Jack),
            PlayingCard::new(Suit::Club, Value::Ten),
            PlayingCard::new(Suit::Diamond, Value::Nine),
            PlayingCard::new(Suit::Club, Value::Eight),
        ]);
        assert_eq!(high_card_hand.cmp(&other_high_card_hand), Ordering::Greater);
    }
}
