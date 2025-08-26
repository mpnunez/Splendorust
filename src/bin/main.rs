use splendorust::cards::{Card, read_cards_from_csv};
use splendorust::state::GameState;

fn main() {
    let cards = read_cards_from_csv("cards.csv").expect("Failed to read cards from CSV");
    println!("Successfully read {} cards from CSV", cards.len());
    
    // Print first few cards as examples
    for (i, card) in cards.iter().take(5).enumerate() {
        println!("Card {}: Tier={}, Color={}, Points={}, Cost={:?}", 
                    i + 1, card.tier, card.color, card.points, card.cost);
    }

    // Separate nobles and gem cards
    let nobles = cards.iter().filter(|c| c.tier == 0);
    let gem_cards = cards.iter().filter(|c| c.tier > 0);

    let game_state = GameState::new(
        vec!["Marcel", "Kun"],
        gem_cards.cloned().collect(),
        nobles.cloned().collect()
    );

}


