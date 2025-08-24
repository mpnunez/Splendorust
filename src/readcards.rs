
use std::collections::HashMap;
use color::Color;
use cost::Tokens;
use card::Card;



// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, serde::Deserialize)]
struct CardRecord {
    #[serde(rename = "Level")]
    level: u8,
    #[serde(rename = "Color")]
    color: String,
    #[serde(rename = "PV")]
    pv: u8,
    #[serde(rename = "Black")]
    black: u8,
    #[serde(rename = "Blue")]
    blue: u8,
    #[serde(rename = "Green")]
    green: u8,
    #[serde(rename = "Red")]
    red: u8,
    #[serde(rename = "White")]
    white: u8,
}

impl CardRecord {

    fn get_color(&self) -> Color {
        match self.color.as_ref() {
            "Black" => Color::Black,
            "Blue" => Color::Blue,
            "Green" => Color::Green,
            "Red" => Color::Red,
            "White" => Color::White,
            _ => Color::Joker,
        }
    }

    fn create_card(&self) -> Card {
        Card {
            color: self.get_color(),
            cost: Tokens {
                black: self.black,
                blue: self.blue,
                green: self.green,
                red: self.red,
                white: self.white,
                joker: 0,
            },
            points: self.pv,
        }
    }
}

pub fn read_cards_of_level(fname: &str, level: u8) -> Vec<Card> {

    // 3 empty decks to be populated
    let mut deck: Vec<Card> = Vec::new();

    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(fname).expect("cards.csv not read");
    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        if result.is_err() {
            continue;
        }
        let record: CardRecord = result.unwrap();
        if record.level != level {
            continue;
        }
        let card = record.create_card();
        deck.push(card.clone());
    }

    deck
}

/// Reads a CSV file and returns a list of maps (String -> i32) counting the integer values in each column.
/// Each map in the returned vector represents the count of each unique integer value for a specific column.
pub fn count_csv_column_values(fname: &str) -> Result<Vec<HashMap<String, i32>>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(fname)?;
    
    // Get headers to know column names
    let headers = rdr.headers()?.clone();
    let num_columns = headers.len();
    
    // Initialize a vector of HashMaps, one for each column
    let mut column_counts: Vec<HashMap<String, i32>> = vec![HashMap::new(); num_columns];
    
    // Process each record
    for result in rdr.records() {
        let record = result?;
        
        // For each field in the record, try to parse as integer and count
        for (col_index, field) in record.iter().enumerate() {
            if col_index < num_columns {
                // Try to parse the field as an integer
                if let Ok(int_value) = field.parse::<i32>() {
                    let count_map = &mut column_counts[col_index];
                    let key = int_value.to_string();
                    *count_map.entry(key).or_insert(0) += 1;
                }
                // If it's not an integer, we skip it (don't count non-integer values)
            }
        }
    }
    
    Ok(column_counts)
}

#[cfg(test)]
mod tests {
    use super::*;

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

}
