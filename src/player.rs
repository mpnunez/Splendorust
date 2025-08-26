use std::collections::HashMap;
use crate::cards::Card;
 
 #[derive(Debug, Clone)]

pub struct Player {
    name: String,
    points: i32,
    tokens: HashMap<String, i32>,
    discounts: HashMap<String, i32>,
    owned_cards: Vec<Card>,
    reserved_cards: Vec<Card>,
}

impl Player {
    pub fn new(name: &str) -> Self {
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

    pub fn total_tokens(&self, color: &str) -> i32 {
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
        if !self.can_afford(&card.get_cost()) {
            return Err("Cannot afford this card".to_string());
        }
        // Deduct tokens (not handling joker logic here for simplicity)
        for (color, &amount) in card.get_cost().iter() {
            let entry = self.tokens.entry(color.clone()).or_insert(0);
            *entry -= amount;
        }
        self.points += card.get_points();
        self.owned_cards.push(card.clone());
        let discount_entry = self.discounts.entry(card.get_color().to_string()).or_insert(0);
        *discount_entry += 1;
        Ok(())
    }

    fn get_points(&self) -> i32 {
        self.points
    }
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
        assert_eq!(kun.get_points(), 1);
        assert_eq!(kun.total_discount("red"), 1);
        assert_eq!(kun.total_tokens("red"), 0);
        assert_eq!(kun.total_tokens("blue"), 0);
        assert_eq!(kun.total_tokens("green"), 0);
    }

}
