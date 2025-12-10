package com.poker.bot.engine;

/**
 * Énumération des rangs de mains au poker (du plus faible au plus fort)
 */
public enum HandRank {
    HIGH_CARD(1, "Carte haute"),
    ONE_PAIR(2, "Paire"),
    TWO_PAIR(3, "Double paire"),
    THREE_OF_A_KIND(4, "Brelan"),
    STRAIGHT(5, "Suite"),
    FLUSH(6, "Couleur"),
    FULL_HOUSE(7, "Full"),
    FOUR_OF_A_KIND(8, "Carré"),
    STRAIGHT_FLUSH(9, "Quinte flush"),
    ROYAL_FLUSH(10, "Quinte flush royale");
    
    private final int value;
    private final String frenchName;
    
    HandRank(int value, String frenchName) {
        this.value = value;
        this.frenchName = frenchName;
    }
    
    public int getValue() {
        return value;
    }
    
    public String getFrenchName() {
        return frenchName;
    }
    
    @Override
    public String toString() {
        return frenchName;
    }
}
