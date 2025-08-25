use std::collections::HashMap;
use std::error::Error;
use csv;

#[derive(Debug, Clone)]
pub struct Card {
    pub points: i32,
    pub cost: HashMap<String, i32>,
    pub color: String,
    pub tier: u8,
}

impl Card {
    pub fn get_points(&self) -> i32 {
        self.points
    }

    pub fn get_color(&self) -> &str {
        &self.color
    }

    pub fn get_tier(&self) -> u8 {
        self.tier
    }

    pub fn get_cost(&self) -> &HashMap<String, i32> {
        &self.cost
    }
}

// Function to read cards from CSV file
pub fn read_cards_from_csv(fname: &str) -> Result<Vec<Card>, Box<dyn Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;

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


}
