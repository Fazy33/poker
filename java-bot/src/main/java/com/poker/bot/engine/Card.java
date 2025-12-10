package com.poker.bot.engine;

import java.util.Objects;

/**
 * Représente une carte de poker
 */
public class Card implements Comparable<Card> {
    private final int rank;  // 2=2, 3=3, ..., 10=10, 11=J, 12=Q, 13=K, 14=A
    private final char suit; // ♠, ♥, ♦, ♣
    
    public Card(int rank, char suit) {
        this.rank = rank;
        this.suit = suit;
    }
    
    /**
     * Parse une carte depuis le format API (ex: "A♠", "K♦", "10♣")
     */
    public static Card fromString(String cardStr) {
        if (cardStr == null || cardStr.length() < 2) {
            throw new IllegalArgumentException("Format de carte invalide: " + cardStr);
        }
        
        // Extraire la couleur (dernier caractère)
        char suit = cardStr.charAt(cardStr.length() - 1);
        
        // Extraire la valeur (tout sauf le dernier caractère)
        String rankStr = cardStr.substring(0, cardStr.length() - 1);
        
        int rank;
        switch (rankStr) {
            case "A": rank = 14; break;
            case "K": rank = 13; break;
            case "Q": rank = 12; break;
            case "J": rank = 11; break;
            default:
                try {
                    rank = Integer.parseInt(rankStr);
                    if (rank < 2 || rank > 10) {
                        throw new IllegalArgumentException("Rang invalide: " + rankStr);
                    }
                } catch (NumberFormatException e) {
                    throw new IllegalArgumentException("Format de rang invalide: " + rankStr);
                }
        }
        
        return new Card(rank, suit);
    }
    
    public int getRank() {
        return rank;
    }
    
    public char getSuit() {
        return suit;
    }
    
    public String getRankString() {
        switch (rank) {
            case 14: return "A";
            case 13: return "K";
            case 12: return "Q";
            case 11: return "J";
            default: return String.valueOf(rank);
        }
    }
    
    @Override
    public String toString() {
        return getRankString() + suit;
    }
    
    @Override
    public int compareTo(Card other) {
        return Integer.compare(this.rank, other.rank);
    }
    
    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Card card = (Card) o;
        return rank == card.rank && suit == card.suit;
    }
    
    @Override
    public int hashCode() {
        return Objects.hash(rank, suit);
    }
}
