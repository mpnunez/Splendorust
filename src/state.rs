use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::cards::Card;
use crate::player::Player;

const MAX_PLAYERS: usize = 4;
const MIN_PLAYERS: usize = 2;
const INITIAL_TOKENS: i32 = 7;
const MAX_TOKENS_PER_COLOR: i32 = 10;
const NUM_CARDS_FOR_PURCHASE: usize = 4; // for each tier
const NUM_NOBLES: usize = 5;
const WINNING_POINTS: i32 = 15;
const COLORS: [&str; 6] = ["red", "blue", "green", "black", "white", "joker"];

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

enum Action {
    AcquireTokens(Vec<String>), // Colors of tokens to acquire
    ReserveCard(Option<Card>),  // Card to reserve (None for random)
    BuyCard(Card),              // Card to buys
    Pass(()),                   // Pass turns
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
    pub fn new(player_names: Vec<&str>, mut cards: Vec<Card>, nobles: Vec<Card>) -> Self {
        let players = player_names.into_iter().map(|name| Player::new(name)).collect();
        let mut bank = HashMap::new();
        for color in &["red", "blue", "green", "black", "white", "joker"] {
            bank.insert(color.to_string(), INITIAL_TOKENS);
        }
        let max_tier = cards.iter().map(|c| c.tier).max().unwrap_or(0);
        let mut decks: Vec<Vec<Card>> = vec![];

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);


        GameState {
            players,
            current_round: 0,
            current_turn: 0,
            tiers: (1..=max_tier as u8).map(|tier| {
                let tier_deck: Vec<Card> = cards.iter().filter(|c| c.get_tier() == tier).cloned().collect();
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

    pub fn refresh_tiers(&mut self) {
        for tier in &mut self.tiers {
            tier.refresh_cards();
        }
    }

    pub fn show_state(&self) {
        println!("Current Round: {}, Current Turn: {}", self.current_round, self.current_turn);
        for (i, player) in self.players.iter().enumerate() {
            println!("Player {}: {:?}", i + 1, player);
        }
        for tier in &self.tiers {
            println!("Tier {}: Available Cards:", tier.tier);
            for (i, card_option) in tier.cards_for_purchase.iter().enumerate() {
                if let Some(card) = card_option {
                    println!("  Slot {}: {:?}", i + 1, card);
                } else {
                    println!("  Slot {}: Empty", i + 1);
                }
            }
        }
        println!("Nobles Available:");
        for noble in &self.nobles {
            println!("  {:?}", noble);
        }
        println!("Bank Tokens:");
        for (color, &amount) in &self.bank {
            println!("  {}: {}", color, amount);
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
