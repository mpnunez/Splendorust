use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::rngs::ThreadRng;

fn main() {
    println!("Hello, world!");
    let mut rng = thread_rng();
}

struct Cost {
    points: i32,
    coins: Vec<i32>, // Assuming coins are represented as integers
}

struct Card {
    points: i32,
    cost: Cost,
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

// trait Cost {
//     fn is_affordable(&self, player: &Player) -> bool {
//         // Check if the cost can be afforded by the player
//         true // Placeholder implementation
//     }
// }

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
