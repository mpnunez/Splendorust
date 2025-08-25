// Example usage of the read_cards_from_csv function
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone)]
struct Card {
    points: i32,
    cost: HashMap<String, i32>,
    color: String,
    tier: u8,
}

struct Player {
    name: String,
    points: i32,
    tokens: HashMap<String, i32>,
    discounts: HashMap<String, i32>,
    owned_cards: Vec<Card>,
    reserved_cards: Vec<Card>,
}

impl Player {
    fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            points: 0,
            tokens: HashMap::new(),
            discounts: HashMap::new(),
            owned_cards: vec![],
            reserved_cards: vec![],
        }
    }

    fn total_discount(&self, color: &str) -> i32 {
        *self.discounts.get(color).unwrap_or(&0)
    }

    fn total_tokens(&self, color: &str) -> i32 {
        *self.tokens.get(color).unwrap_or(&0)
    }

    fn can_afford(&self, cost: &HashMap<String, i32>) -> bool {
        for (color, &amount) in cost.iter() {
            let total_available = self.total_tokens(color) + self.total_discount(color);
            if total_available < amount {
                return false;
            }
        }
        true
    }

    fn acquire_token(&mut self, color: &str) {
        let entry = self.tokens.entry(color.to_string()).or_insert(0);
        *entry += 1;
    }

    fn buy_card(&mut self, card: &Card) -> Result<(), String> {
        if !self.can_afford(&card.cost) {
            return Err("Cannot afford this card".to_string());
        }
        // Deduct tokens (not handling joker logic here for simplicity)
        for (color, &amount) in card.cost.iter() {
            let entry = self.tokens.entry(color.clone()).or_insert(0);
            *entry -= amount;
        }
        self.points += card.points;
        self.owned_cards.push(card.clone());
        let discount_entry = self.discounts.entry(card.color.clone()).or_insert(0);
        *discount_entry += 1;
        Ok(())
    }
}

// Function to read cards from CSV file
fn read_cards_from_csv(fname: &str) -> Result<Vec<Card>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(fname)?;
    
    // Get headers to know column names
    let headers = rdr.headers()?.clone();
    let mut cards: Vec<Card> = Vec::new();

    // Process each record
    for result in rdr.records() {
        let record = result?;
        
        // Initialize variables for the Card struct
        let mut tier: u8 = 0;
        let mut color = String::new();
        let mut points: i32 = 0;
        let mut cost: HashMap<String, i32> = HashMap::new();
        
        // Process each field in the record
        for (col_index, field) in record.iter().enumerate() {
            let header_option = headers.get(col_index);
            if header_option.is_none() {
                continue; // Skip if no header found
            }
            let header = header_option.unwrap();

            match header {
                "Tier" => {
                    tier = field.parse::<u8>().unwrap_or(0);
                },
                "Color" => {
                    color = field.to_string();
                },
                "PV" => {
                    points = field.parse::<i32>().unwrap_or(0);
                },
                // All other columns go into the cost HashMap
                _ => {
                    let value = field.parse::<i32>().unwrap_or(0);
                    cost.insert(header.to_string(), value);
                }
            }

        }
        
        // Create and add the card
        let card = Card {
            points,
            cost,
            color,
            tier,
        };
        cards.push(card);
    }
    
    Ok(cards)
}

fn main() {
    let cards = read_cards_from_csv("cards.csv").expect("Failed to read cards from CSV");
    println!("Successfully read {} cards from CSV", cards.len());
    
    // Print first few cards as examples
    for (i, card) in cards.iter().take(5).enumerate() {
        println!("Card {}: Tier={}, Color={}, Points={}, Cost={:?}", 
                    i + 1, card.tier, card.color, card.points, card.cost);
    }

    let mut decks: Vec<Vec<Card>> = vec![];
    for tier in 1..=3 {
        let mut deck: Vec<Card> = cards.iter().filter(|c| c.tier == tier).cloned().collect();
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
        decks.push(deck);
    }

    let nobles = cards.iter().filter(|c| c.tier == 0);

    let mut marcel = Player::new("Marcel");
    let mut kun = Player::new("Kun");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rstest::fixture;
    use std::collections::HashMap;

    #[fixture]
    pub fn card() -> Card {
        Card {
            points: 1,
            color: String::from("red"),
            tier: 1,
            cost: HashMap::from([
                (String::from("red"), 1),
                (String::from("blue"), 2),
                (String::from("green"), 1),
            ]),
        }
    }

    #[test]
    fn test_read_csv () {

        let cards = read_cards_from_csv("cards.csv").unwrap();

        let tier1 = cards.iter().filter(|c| c.tier == 1);
        let tier2 = cards.iter().filter(|c| c.tier == 2);
        let tier3 = cards.iter().filter(|c| c.tier == 3);
        let nobles = cards.iter().filter(|c| c.tier == 0);

        assert_eq!(cards.len(), 99);
        assert_eq!(tier1.count(), 40);
        assert_eq!(tier2.count(), 30);
        assert_eq!(tier3.count(), 20);
        assert_eq!(nobles.count(), 9);
    }

    #[rstest]
    #[test]
    fn test_cannot_buy_card (card: Card) {

        let mut marcel = Player::new("Marcel");
        assert!(marcel.buy_card(&card).is_err());
    }

    #[rstest]
    #[test]
    fn test_buy_card (card: Card) {
        let mut kun = Player::new("Kun");
        kun.acquire_token("red");
        kun.acquire_token("blue");
        kun.acquire_token("blue");
        kun.acquire_token("green");
        assert!(kun.buy_card(&card).is_ok());
    }

}
