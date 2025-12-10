package com.poker.bot.engine;

import org.junit.Test;
import static org.junit.Assert.*;

import java.util.Arrays;
import java.util.List;

/**
 * Tests pour la classe Card
 */
public class CardTest {

    @Test
    public void testParseSimpleCards() {
        Card aceSpades = Card.fromString("A♠");
        assertEquals(14, aceSpades.getRank());
        assertEquals('♠', aceSpades.getSuit());

        Card kingHearts = Card.fromString("K♥");
        assertEquals(13, kingHearts.getRank());
        assertEquals('♥', kingHearts.getSuit());

        Card twoClubs = Card.fromString("2♣");
        assertEquals(2, twoClubs.getRank());
        assertEquals('♣', twoClubs.getSuit());
    }

    @Test
    public void testParseTenCard() {
        Card tenDiamonds = Card.fromString("10♦");
        assertEquals(10, tenDiamonds.getRank());
        assertEquals('♦', tenDiamonds.getSuit());
    }

    @Test
    public void testCardToString() {
        Card card = new Card(14, '♠');
        assertEquals("A♠", card.toString());

        Card card2 = new Card(10, '♥');
        assertEquals("10♥", card2.toString());
    }

    @Test
    public void testCardComparison() {
        Card ace = new Card(14, '♠');
        Card king = new Card(13, '♠');
        Card two = new Card(2, '♣');

        assertTrue(ace.compareTo(king) > 0);
        assertTrue(king.compareTo(two) > 0);
        assertTrue(two.compareTo(ace) < 0);
    }

    @Test
    public void testCardEquality() {
        Card card1 = new Card(14, '♠');
        Card card2 = new Card(14, '♠');
        Card card3 = new Card(14, '♥');

        assertEquals(card1, card2);
        assertNotEquals(card1, card3);
    }
}
