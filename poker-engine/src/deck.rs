use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Représente un paquet de cartes
#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Crée un nouveau paquet de 52 cartes
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in Rank::all() {
                cards.push(Card::new(rank, suit));
            }
        }
        
        Deck { cards }
    }

    /// Mélange le paquet (algorithme Fisher-Yates)
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Distribue une carte du dessus du paquet
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Distribue plusieurs cartes
    pub fn deal_multiple(&mut self, count: usize) -> Vec<Card> {
        let mut dealt = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(card) = self.deal() {
                dealt.push(card);
            }
        }
        dealt
    }

    /// Retourne le nombre de cartes restantes
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }

    /// Réinitialise le paquet avec toutes les cartes
    pub fn reset(&mut self) {
        *self = Deck::new();
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_deck() {
        let deck = Deck::new();
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_deal() {
        let mut deck = Deck::new();
        let card = deck.deal();
        assert!(card.is_some());
        assert_eq!(deck.remaining(), 51);
    }

    #[test]
    fn test_deal_multiple() {
        let mut deck = Deck::new();
        let cards = deck.deal_multiple(5);
        assert_eq!(cards.len(), 5);
        assert_eq!(deck.remaining(), 47);
    }

    #[test]
    fn test_shuffle() {
        let mut deck1 = Deck::new();
        let mut deck2 = Deck::new();
        
        deck1.shuffle();
        
        // Les paquets mélangés devraient être différents (très probable)
        let cards1: Vec<_> = (0..52).map(|_| deck1.deal().unwrap()).collect();
        let cards2: Vec<_> = (0..52).map(|_| deck2.deal().unwrap()).collect();
        
        assert_ne!(cards1, cards2);
    }

    #[test]
    fn test_reset() {
        let mut deck = Deck::new();
        deck.deal_multiple(10);
        assert_eq!(deck.remaining(), 42);
        
        deck.reset();
        assert_eq!(deck.remaining(), 52);
    }

    #[test]
    fn test_deal_empty_deck() {
        let mut deck = Deck::new();
        deck.deal_multiple(52);
        assert_eq!(deck.remaining(), 0);
        assert!(deck.deal().is_none());
    }
}
