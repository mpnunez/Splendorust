use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::rngs::ThreadRng;
use std::collections::HashMap;

const JOKER: &str = "joker";

fn main() {
    println!("Hello, world!");
    
}


trait Amount {
    fn get_amount(&self, color: &str) -> i32;
    fn can_be_afforded_by(&self, cost: &impl Amount) -> bool;
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
}

struct Card {
    points: i32,
    //cost: impl Amount,
}

trait Deck {
    fn draw_card(&mut self) -> Option<Card>;
    fn shuffle(&mut self, rng: &mut ThreadRng);
}

impl Deck for Vec<Card> {
    fn draw_card(&mut self) -> Option<Card> {
        return self.pop();
    }

    fn shuffle(&mut self, rng: &mut ThreadRng) {
        self.shuffle(rng);
    }
}

struct CardTier {
    pub cards_for_purchase: Vec<Card>,
}

trait Player {
    fn buy_card(&self, card: &Card) -> Result<(), String> {
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
    fn is_terminaal(&self) -> bool {
        // Check if the game has ended
        false // Placeholder implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_possible_moves() {
        let mut rng = thread_rng();

        let mut cost: HashMap<String, i32> = HashMap::new();

        // Insert key-value pairs
        cost.insert(String::from("red"), 1);
        cost.insert(String::from("blue"), 2);
        cost.insert(String::from("green"), 3);

        assert_eq!(cost.get_amount("red"), 1);
        assert_eq!(cost.get_amount("black"), 0);

        let mut bank: HashMap<String, i32> = HashMap::new();

        // Insert key-value pairs
        bank.insert(String::from("red"), 1);
        bank.insert(String::from("blue"), 1);
        bank.insert(String::from("green"), 1);

        assert!(!cost.can_be_afforded_by(&bank));

        let mut bank2: HashMap<String, i32> = HashMap::new();

        // Insert key-value pairs
        bank2.insert(String::from("red"), 4);
        bank2.insert(String::from("blue"), 4);
        bank2.insert(String::from("green"), 4);

        assert!(bank2.can_be_afforded_by(&bank2));
    }
}
