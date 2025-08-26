use splendorust::cards::{Card, read_cards_from_csv};
use splendorust::state::GameState;

fn main() {
    let cards = read_cards_from_csv("cards.csv").expect("Failed to read cards from CSV");

    // Separate nobles and gem cards
    let nobles = cards.iter().filter(|c| c.tier == 0);
    let gem_cards = cards.iter().filter(|c| c.tier > 0);

    let mut game_state = GameState::new(
        vec!["Marcel", "Kun"],
        gem_cards.cloned().collect(),
        nobles.cloned().collect()
    );

    game_state.refresh_tiers();
    game_state.show_state();

}


