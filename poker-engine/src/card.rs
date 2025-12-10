use serde::{Deserialize, Serialize};
use std::fmt;

/// Représente les couleurs des cartes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suit {
    Hearts,   // Coeur
    Diamonds, // Carreau
    Clubs,    // Trèfle
    Spades,   // Pique
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        };
        write!(f, "{}", symbol)
    }
}

/// Représente les valeurs des cartes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Rank {
    /// Retourne toutes les valeurs possibles
    pub fn all() -> [Rank; 13] {
        [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ]
    }

    /// Retourne la valeur numérique de la carte
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        write!(f, "{}", s)
    }
}

/// Représente une carte de jeu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    /// Crée une nouvelle carte
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new(Rank::Ace, Suit::Spades);
        assert_eq!(card.rank, Rank::Ace);
        assert_eq!(card.suit, Suit::Spades);
    }

    #[test]
    fn test_card_display() {
        let card = Card::new(Rank::King, Suit::Hearts);
        assert_eq!(format!("{}", card), "K♥");
    }

    #[test]
    fn test_rank_ordering() {
        assert!(Rank::Ace > Rank::King);
        assert!(Rank::Two < Rank::Three);
    }

    #[test]
    fn test_rank_value() {
        assert_eq!(Rank::Ace.value(), 14);
        assert_eq!(Rank::Two.value(), 2);
    }
}
