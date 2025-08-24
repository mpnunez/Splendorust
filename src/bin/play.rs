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

