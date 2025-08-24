use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::rngs::ThreadRng;
use std::collections::HashMap;

const JOKER: &str = "joker";

fn main() {

    // make a player
    let player = RandomPlayer{};

    // give player a bank
    let mut bank: HashMap<String, i32> = HashMap::new();
    bank.insert(String::from("red"), 1);
    bank.insert(String::from("blue"), 1);
    bank.insert(String::from("green"), 1);
    bank.insert(String::from("black"), 1);
    bank.insert(String::from("white"), 1);


    // make a card
    let mut cost: HashMap<String, i32> = HashMap::new();
    cost.insert(String::from("red"), 1);
    cost.insert(String::from("blue"), 2);
    cost.insert(String::from("green"), 3);

    // make a card with the cost
    let card = Card {
        points: 1,
        cost: cost,
        discount_provided: HashMap::new(),
    };

    // try to buy the card
    //match player.buy_card(&card) {
    //    Ok(_) => println!("Player bought the card!"),
    //    Err(e) => println!("Player could not buy the card: {}", e),
    //}
    
}


trait Amount {
    fn get_amount(&self, color: &str) -> i32;
    fn can_be_afforded_by(&self, cost: &impl Amount) -> bool;
    fn add(&self, other: &Self) -> Self;
    fn add_to(&mut self, other: &Self);
    fn sub(&self, other: &Self) -> Self;
    fn sub_from(&mut self, other: &Self);
}

impl Amount for HashMap<String, i32> {
    fn get_amount(&self, color: &str) -> i32 {
        self.get(color).unwrap_or(&0).clone()
    }
    fn can_be_afforded_by(&self, bank: &impl Amount) -> bool {
        for (color, value) in self.iter() {
            if bank.get_amount(color) < *value {
                return false;
            }
        }
        return true;
    }
    fn add(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for (color, value) in other.iter() {
            *result.entry(color.clone()).or_insert(0) += value;
        }
        result
    }

    fn sub(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for (color, value) in other.iter() {
            *result.entry(color.clone()).or_insert(0) -= value;
        }
        result
    }
}

struct Card<T: Amount> {
    points: i32,
    cost: T,
    discount_provided: T,
}

trait Deck<T: Amount> {
    fn draw_card(&mut self) -> Option<Card<T>>;
    fn shuffle(&mut self, rng: &mut ThreadRng);
}

impl<T: Amount> Deck<T> for Vec<Card<T>> {
    fn draw_card(&mut self) -> Option<Card<T>> {
        return self.pop();
    }

    fn shuffle(&mut self, rng: &mut ThreadRng) {
        self.shuffle(rng);
    }
}

struct CardTier<T: Amount> {
    pub cards_for_purchase: Vec<Card<T>>,
}

trait Player {
    fn buy_card<T: Amount>(&self, card: &Card<T>) -> Result<(), String> {
        // if card.cost.is_affordable(self) {
        //     // Logic to buy the card
        //     Ok(())
        // } else {
        //     Err("Cannot afford this card".to_string())
        // }
        return Ok(()); // Placeholder implementation
    }

}

struct RandomPlayer {

}

impl Player for RandomPlayer {

}

struct GameState {
    
}

impl GameState {
    fn is_terminal(&self) -> bool {
        // Check if the game has ended
        false // Placeholder implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{rstest,fixture};

    #[fixture]
    pub fn cost() -> HashMap<String, i32> {
        let mut c: HashMap<String, i32> = HashMap::new();
        c.insert(String::from("red"), 1);
        c.insert(String::from("blue"), 2);
        c.insert(String::from("green"), 3);
        c
    }

    #[fixture]
    pub fn bank() -> HashMap<String, i32> {
        let mut b: HashMap<String, i32> = HashMap::new();
        b.insert(String::from("red"), 1);
        b.insert(String::from("blue"), 1);
        b.insert(String::from("green"), 1);
        b
    }

    #[fixture]
    pub fn bank2() -> HashMap<String, i32> {
        let mut b: HashMap<String, i32> = HashMap::new();
        b.insert(String::from("red"), 4);
        b.insert(String::from("blue"), 4);
        b.insert(String::from("green"), 4);
        b
    }

    #[rstest]
    #[test]
    fn can_afford(
        cost: HashMap<String, i32>,
        bank: HashMap<String, i32>,
        bank2: HashMap<String, i32>
    ) {
        assert_eq!(cost.get_amount("red"), 1);
        assert_eq!(cost.get_amount("black"), 0);
        assert!(!cost.can_be_afforded_by(&bank));
        assert!(cost.can_be_afforded_by(&bank2));
    }

    #[rstest]
    #[test]
    fn add_and_sub(
        cost: HashMap<String, i32>,
        bank: HashMap<String, i32>,
        bank2: HashMap<String, i32>
    ) {
        let result = cost.add(&bank);
        assert_eq!(result.get_amount("red"), 2);
        assert_eq!(result.get_amount("blue"), 3);
        assert_eq!(result.get_amount("green"), 4);
        assert_eq!(result.get_amount("black"), 0);

        let result2 = bank2.sub(&cost);
        assert_eq!(result2.get_amount("red"), 3);
        assert_eq!(result2.get_amount("blue"), 2);
        assert_eq!(result2.get_amount("green"), 1);
    }
}
