use crate::card::{Card, Rank};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;

/// Représente les différents types de mains au poker (du plus faible au plus fort)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

/// Représente une main évaluée avec son rang et ses cartes de kicker
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hand {
    pub rank: HandRank,
    pub cards: Vec<Card>,
    pub kickers: Vec<Rank>, // Pour départager les mains de même rang
}

impl Hand {
    /// Évalue la meilleure main possible à partir de 5 à 7 cartes
    pub fn evaluate(cards: &[Card]) -> Self {
        assert!(cards.len() >= 5 && cards.len() <= 7, "Il faut entre 5 et 7 cartes");

        // Si on a 7 cartes, on teste toutes les combinaisons de 5 cartes
        if cards.len() == 7 {
            return Self::best_five_card_hand(cards);
        }

        // Sinon on évalue directement les 5 cartes
        Self::evaluate_five_cards(cards)
    }

    /// Trouve la meilleure main de 5 cartes parmi 7 cartes
    fn best_five_card_hand(cards: &[Card]) -> Self {
        let mut best_hand = None;

        // Générer toutes les combinaisons de 5 cartes parmi 7
        for i in 0..cards.len() {
            for j in (i + 1)..cards.len() {
                let mut five_cards = Vec::new();
                for (k, card) in cards.iter().enumerate() {
                    if k != i && k != j {
                        five_cards.push(*card);
                    }
                }
                
                let hand = Self::evaluate_five_cards(&five_cards);
                
                best_hand = Some(match best_hand {
                    None => hand,
                    Some(current_best) => {
                        if hand > current_best {
                            hand
                        } else {
                            current_best
                        }
                    }
                });
            }
        }

        best_hand.unwrap()
    }

    /// Évalue exactement 5 cartes
    fn evaluate_five_cards(cards: &[Card]) -> Self {
        assert_eq!(cards.len(), 5, "Il faut exactement 5 cartes");

        let mut sorted_cards = cards.to_vec();
        sorted_cards.sort_by(|a, b| b.rank.cmp(&a.rank)); // Tri décroissant

        // Vérifier les différentes combinaisons
        if let Some(hand) = Self::check_royal_flush(&sorted_cards) {
            return hand;
        }
        if let Some(hand) = Self::check_straight_flush(&sorted_cards) {
            return hand;
        }
        if let Some(hand) = Self::check_four_of_a_kind(&sorted_cards) {
            return hand;
        }
        if let Some(hand) = Self::check_full_house(&sorted_cards) {
            return hand;
        }
        if let Some(hand) = Self::check_flush(&sorted_cards) {
            return hand;
        }
        if let Some(hand) = Self::check_straight(&sorted_cards) {
            return hand;
        }
        if let Some(hand) = Self::check_three_of_a_kind(&sorted_cards) {
            return hand;
        }
        if let Some(hand) = Self::check_two_pair(&sorted_cards) {
            return hand;
        }
        if let Some(hand) = Self::check_one_pair(&sorted_cards) {
            return hand;
        }

        // Carte haute par défaut
        Self {
            rank: HandRank::HighCard,
            cards: sorted_cards.clone(),
            kickers: sorted_cards.iter().map(|c| c.rank).collect(),
        }
    }

    fn check_royal_flush(cards: &[Card]) -> Option<Self> {
        if let Some(hand) = Self::check_straight_flush(cards) {
            if hand.kickers[0] == Rank::Ace {
                return Some(Hand {
                    rank: HandRank::RoyalFlush,
                    cards: hand.cards,
                    kickers: hand.kickers,
                });
            }
        }
        None
    }

    fn check_straight_flush(cards: &[Card]) -> Option<Self> {
        if Self::is_flush(cards) && Self::is_straight(cards) {
            return Some(Hand {
                rank: HandRank::StraightFlush,
                cards: cards.to_vec(),
                kickers: vec![cards[0].rank],
            });
        }
        None
    }

    fn check_four_of_a_kind(cards: &[Card]) -> Option<Self> {
        let rank_counts = Self::count_ranks(cards);
        
        for (rank, count) in rank_counts.iter() {
            if *count == 4 {
                let kicker = cards.iter()
                    .find(|c| c.rank != *rank)
                    .map(|c| c.rank)
                    .unwrap();
                
                return Some(Hand {
                    rank: HandRank::FourOfAKind,
                    cards: cards.to_vec(),
                    kickers: vec![*rank, kicker],
                });
            }
        }
        None
    }

    fn check_full_house(cards: &[Card]) -> Option<Self> {
        let rank_counts = Self::count_ranks(cards);
        
        let mut three = None;
        let mut two = None;
        
        for (rank, count) in rank_counts.iter() {
            if *count == 3 {
                three = Some(*rank);
            } else if *count == 2 {
                two = Some(*rank);
            }
        }
        
        if let (Some(three_rank), Some(two_rank)) = (three, two) {
            return Some(Hand {
                rank: HandRank::FullHouse,
                cards: cards.to_vec(),
                kickers: vec![three_rank, two_rank],
            });
        }
        None
    }

    fn check_flush(cards: &[Card]) -> Option<Self> {
        if Self::is_flush(cards) {
            return Some(Hand {
                rank: HandRank::Flush,
                cards: cards.to_vec(),
                kickers: cards.iter().map(|c| c.rank).collect(),
            });
        }
        None
    }

    fn check_straight(cards: &[Card]) -> Option<Self> {
        if Self::is_straight(cards) {
            return Some(Hand {
                rank: HandRank::Straight,
                cards: cards.to_vec(),
                kickers: vec![cards[0].rank],
            });
        }
        None
    }

    fn check_three_of_a_kind(cards: &[Card]) -> Option<Self> {
        let rank_counts = Self::count_ranks(cards);
        
        for (rank, count) in rank_counts.iter() {
            if *count == 3 {
                let mut kickers: Vec<Rank> = cards.iter()
                    .filter(|c| c.rank != *rank)
                    .map(|c| c.rank)
                    .collect();
                kickers.sort_by(|a, b| b.cmp(a));
                
                let mut result_kickers = vec![*rank];
                result_kickers.extend(kickers);
                
                return Some(Hand {
                    rank: HandRank::ThreeOfAKind,
                    cards: cards.to_vec(),
                    kickers: result_kickers,
                });
            }
        }
        None
    }

    fn check_two_pair(cards: &[Card]) -> Option<Self> {
        let rank_counts = Self::count_ranks(cards);
        
        let mut pairs: Vec<Rank> = rank_counts.iter()
            .filter(|(_, count)| **count == 2)
            .map(|(rank, _)| *rank)
            .collect();
        
        if pairs.len() == 2 {
            pairs.sort_by(|a, b| b.cmp(a));
            
            let kicker = cards.iter()
                .find(|c| c.rank != pairs[0] && c.rank != pairs[1])
                .map(|c| c.rank)
                .unwrap();
            
            return Some(Hand {
                rank: HandRank::TwoPair,
                cards: cards.to_vec(),
                kickers: vec![pairs[0], pairs[1], kicker],
            });
        }
        None
    }

    fn check_one_pair(cards: &[Card]) -> Option<Self> {
        let rank_counts = Self::count_ranks(cards);
        
        for (rank, count) in rank_counts.iter() {
            if *count == 2 {
                let mut kickers: Vec<Rank> = cards.iter()
                    .filter(|c| c.rank != *rank)
                    .map(|c| c.rank)
                    .collect();
                kickers.sort_by(|a, b| b.cmp(a));
                
                let mut result_kickers = vec![*rank];
                result_kickers.extend(kickers);
                
                return Some(Hand {
                    rank: HandRank::OnePair,
                    cards: cards.to_vec(),
                    kickers: result_kickers,
                });
            }
        }
        None
    }

    fn is_flush(cards: &[Card]) -> bool {
        let first_suit = cards[0].suit;
        cards.iter().all(|c| c.suit == first_suit)
    }

    fn is_straight(cards: &[Card]) -> bool {
        let mut ranks: Vec<u8> = cards.iter().map(|c| c.rank.value()).collect();
        ranks.sort_by(|a, b| b.cmp(a));
        
        // Vérifier la suite normale
        let is_normal_straight = ranks.windows(2).all(|w| w[0] == w[1] + 1);
        
        // Vérifier la suite A-2-3-4-5 (roue)
        let is_wheel = ranks == vec![14, 5, 4, 3, 2];
        
        is_normal_straight || is_wheel
    }

    fn count_ranks(cards: &[Card]) -> HashMap<Rank, usize> {
        let mut counts = HashMap::new();
        for card in cards {
            *counts.entry(card.rank).or_insert(0) += 1;
        }
        counts
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // D'abord comparer le rang de la main
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => {
                // Si même rang, comparer les kickers
                self.kickers.cmp(&other.kickers)
            }
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Suit;

    #[test]
    fn test_royal_flush() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
        ];
        let hand = Hand::evaluate(&cards);
        assert_eq!(hand.rank, HandRank::RoyalFlush);
    }

    #[test]
    fn test_straight_flush() {
        let cards = vec![
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
        ];
        let hand = Hand::evaluate(&cards);
        assert_eq!(hand.rank, HandRank::StraightFlush);
    }

    #[test]
    fn test_four_of_a_kind() {
        let cards = vec![
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
        ];
        let hand = Hand::evaluate(&cards);
        assert_eq!(hand.rank, HandRank::FourOfAKind);
    }

    #[test]
    fn test_full_house() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::King, Suit::Spades),
        ];
        let hand = Hand::evaluate(&cards);
        assert_eq!(hand.rank, HandRank::FullHouse);
    }

    #[test]
    fn test_flush() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Clubs),
            Card::new(Rank::Nine, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
        ];
        let hand = Hand::evaluate(&cards);
        assert_eq!(hand.rank, HandRank::Flush);
    }

    #[test]
    fn test_straight() {
        let cards = vec![
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Six, Suit::Spades),
        ];
        let hand = Hand::evaluate(&cards);
        assert_eq!(hand.rank, HandRank::Straight);
    }

    #[test]
    fn test_three_of_a_kind() {
        let cards = vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
        ];
        let hand = Hand::evaluate(&cards);
        assert_eq!(hand.rank, HandRank::ThreeOfAKind);
    }

    #[test]
    fn test_two_pair() {
        let cards = vec![
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
        ];
        let hand = Hand::evaluate(&cards);
        assert_eq!(hand.rank, HandRank::TwoPair);
    }

    #[test]
    fn test_one_pair() {
        let cards = vec![
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Seven, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
        ];
        let hand = Hand::evaluate(&cards);
        assert_eq!(hand.rank, HandRank::OnePair);
    }

    #[test]
    fn test_high_card() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
        ];
        let hand = Hand::evaluate(&cards);
        assert_eq!(hand.rank, HandRank::HighCard);
    }

    #[test]
    fn test_hand_comparison() {
        let flush = Hand {
            rank: HandRank::Flush,
            cards: vec![],
            kickers: vec![Rank::Ace],
        };
        let straight = Hand {
            rank: HandRank::Straight,
            cards: vec![],
            kickers: vec![Rank::Ace],
        };
        assert!(flush > straight);
    }
}
