use crate::hand::*;
use std::cmp::Ordering;
use std::io;

pub mod hand;

fn main() {
    let mut house_hand: Hand = Hand::default();
    let mut player_hand: Hand = Hand::default();
    let mut deck = Hand::generate_deck();

    for _i in 0..5 {
        deck.deal(&mut player_hand);
    }

    for _i in 0..5 {
        deck.deal(&mut house_hand);
    }
    loop {
        println!("Your hand is: {}", player_hand);
        println!("Do you want to swap out any cards? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_uppercase();
        match input.as_str() {
            "Y" => {
                println!("Enter the index of the cards you want to remove, seperated with spaces");
                let mut player_input = String::new();
                io::stdin().read_line(&mut player_input).expect("idk");
                println!("{}", player_input);
                let nums: Vec<usize> = player_input
                    .trim()
                    .split(' ')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                player_hand.group_discard(nums.clone());
                for _i in 0..nums.len() {
                    deck.deal(&mut player_hand);
                }
                break;
            }
            "N" => {
                break;
            }
            _ => continue,
        }
    }
    println!("You have {}.", player_hand);
    println!("They have {}.", house_hand);
    let result = player_hand.cmp(&house_hand);
    match result {
        Ordering::Greater => {
            println!("You win");
        }
        Ordering::Equal => {
            println!("Draw...")
        }
        Ordering::Less => {
            println!("House wins...")
        }
    }
}

#[cfg(test)]
mod test {

    use crate::hand::Hand;

    use super::*;

    #[test]
    fn deck_test() {
        let mut deck = Hand::generate_deck();
        assert_eq!(deck.get_hand().len(), 52);
        // Tests to see if you can add more than 52 cards to a deck
        // If Err is returned, test passes.
        if let Err(err) = deck.draw(hand::card::PlayingCard::new(
            hand::card::Suit::Club,
            hand::card::Value::Ace,
        )) {
            println!("{}", err);
            assert!(true);
        } else {
            assert!(false);
        }
        // Checks to see if deck discards successfully when a card is present.
        // Unsuccessful case is tested in the hand_test test.
        if let Some(_a) = deck.discard(0) {
            assert!(true);
        } else {
            assert!(false);
        }
        assert_eq!(deck.get_hand().len(), 51);
    }

    #[test]
    fn hand_function_test() {
        // Test default constructor

        // Test constructor

        // Test getters

        // Test setters

        // Test successful discard

        // Test unsuccessful discard
    }

    #[test]
    fn scenario_one() {
        let mut deck = Hand::generate_deck();
        let hand_1 = Hand::default();
        let mut hand_arr: [Hand; 5] = [
            hand_1.clone(),
            hand_1.clone(),
            hand_1.clone(),
            hand_1.clone(),
            hand_1.clone(),
        ];
        for _i in 0..5 {
            let _ = hand_arr
                .iter_mut()
                .map(|c| c.draw(deck.discard(0).unwrap()));
        }
        let _ = hand_arr.iter().map(|c| println!("{}", c));
    }
}
