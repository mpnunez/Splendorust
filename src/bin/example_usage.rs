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
            if let Some(header) = headers.get(col_index) {
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
    match read_cards_from_csv("cards.csv") {
        Ok(cards) => {
            println!("Successfully read {} cards from CSV", cards.len());
            
            // Print first few cards as examples
            for (i, card) in cards.iter().take(5).enumerate() {
                println!("Card {}: Tier={}, Color={}, Points={}, Cost={:?}", 
                         i + 1, card.tier, card.color, card.points, card.cost);
            }
            
            // Print some statistics
            let tier1_count = cards.iter().filter(|c| c.tier == 1).count();
            let tier2_count = cards.iter().filter(|c| c.tier == 2).count();
            let tier3_count = cards.iter().filter(|c| c.tier == 3).count();
            let noble_count = cards.iter().filter(|c| c.tier == 0).count();
            
            println!("\nCard distribution:");
            println!("Tier 1: {} cards", tier1_count);
            println!("Tier 2: {} cards", tier2_count);
            println!("Tier 3: {} cards", tier3_count);
            println!("Nobles: {} cards", noble_count);
        },
        Err(e) => {
            eprintln!("Error reading cards from CSV: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
/*
    #[test]
    fn create_decks () {
        let deck1 = read_cards_of_level("cards.csv",1);
        assert_eq!(deck1.len(),40);
        let deck2 = read_cards_of_level("cards.csv",2);
        assert_eq!(deck2.len(),30);
        let deck3 = read_cards_of_level("cards.csv",3);
        assert_eq!(deck3.len(),20);
        let nobles = read_cards_of_level("cards.csv",0);
        assert_eq!(nobles.len(),9);
    }

    #[test]
    fn test_count_csv_column_values() {
        let result = count_csv_column_values("cards.csv").expect("Failed to read CSV");
        
        // Should have 8 columns (Level, Color, PV, Black, Blue, Green, Red, White)
        assert_eq!(result.len(), 8);
        
        // Column 0 (Level) should have counts for levels 0, 1, 2, 3
        let level_counts = &result[0];
        assert!(level_counts.contains_key("0")); // Noble cards
        assert!(level_counts.contains_key("1")); // Level 1 cards
        assert!(level_counts.contains_key("2")); // Level 2 cards
        assert!(level_counts.contains_key("3")); // Level 3 cards
        
        // Column 1 (Color) should be empty since it contains strings, not integers
        let color_counts = &result[1];
        assert!(color_counts.is_empty());
        
        // Column 2 (PV) should have various point values
        let pv_counts = &result[2];
        assert!(pv_counts.contains_key("0")); // Cards with 0 points
        assert!(pv_counts.contains_key("1")); // Cards with 1 point
        
        // Verify we have the expected total number of level 1 cards (40)
        assert_eq!(level_counts.get("1").unwrap_or(&0), &40);
        
        println!("Column counts: {:?}", result);
    }
*/
}
