use crate::cards::Card;
use crate::player::Player;
use std::collections::HashMap;

const MAX_PLAYERS: usize = 4;
const MIN_PLAYERS: usize = 2;
const INITIAL_TOKENS: i32 = 7;
const MAX_TOKENS_PER_COLOR: i32 = 10;
const NUM_CARDS_FOR_PURCHASE: usize = 4; // for each tiers
const NUM_NOBLES: usize = 5;
const WINNING_POINTS: i32 = 15;
const COLORS: [&str; 6] = ["red", "blue", "green", "black", "white", "joker"];
const NUM_TIERS: usize = 3;

/* Deck of cards for a specific tier, along with the cards currently available for purchase */
struct Tier {
    tier: u8,
    deck: Vec<Card>,
    cards_for_purchase: Vec<Option<Card>>,
}

impl Tier {
    fn new(tier: u8, deck: Vec<Card>) -> Self {
        Tier {
            tier,
            deck,
            cards_for_purchase: vec![None; NUM_CARDS_FOR_PURCHASE],
        }
    }

    fn refresh_cards(&mut self) {
        for slot in self.cards_for_purchase.iter_mut() {
            if slot.is_none() {
                if let Some(card) = self.deck.pop() {
                    *slot = Some(card);
                }
            }
        }
    }
}

pub struct GameState {
    players: Vec<Player>,
    current_round: usize,
    current_turn: usize,
    tiers: Vec<Tier>,
    nobles: Vec<Card>,
    bank: HashMap<String, i32>, // Tokens available in the bank
}

impl GameState {
    pub fn new(player_names: Vec<&str>, decks: Vec<Card>, nobles: Vec<Card>) -> Self {
        let players = player_names.into_iter().map(|name| Player::new(name)).collect();
        let mut bank = HashMap::new();
        for color in &["red", "blue", "green", "black", "white", "joker"] {
            bank.insert(color.to_string(), INITIAL_TOKENS);
        }
        GameState {
            players,
            current_round: 0,
            current_turn: 0,
            tiers: (1..=NUM_TIERS as u8).map(|tier| {
                let tier_deck: Vec<Card> = decks.iter().filter(|c| c.get_tier() == tier).cloned().collect();
                Tier::new(tier, tier_deck)
            }).collect(),
            nobles,
            bank,
        }
    }

    fn next_turn(&mut self) {
        self.current_turn = (self.current_turn + 1) % self.players.len();
        if self.current_turn == 0 {
            self.current_round += 1;
        }
    }

    fn current_player(&self) -> &Player {
        &self.players[self.current_turn]
    }

    fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_turn]
    }

    fn refresh_tiers(&mut self) {
        for tier in &mut self.tiers {
            tier.refresh_cards();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state () {
        assert!(true);
    }


}
