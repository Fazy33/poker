package com.poker.bot.engine;

import org.junit.Test;
import static org.junit.Assert.*;

import java.util.Arrays;
import java.util.List;

/**
 * Tests pour la classe HandEvaluator
 */
public class HandEvaluatorTest {

    @Test
    public void testRoyalFlush() {
        List<Card> cards = Arrays.asList(
                new Card(14, '♠'), // A♠
                new Card(13, '♠'), // K♠
                new Card(12, '♠'), // Q♠
                new Card(11, '♠'), // J♠
                new Card(10, '♠') // 10♠
        );

        assertEquals(HandRank.ROYAL_FLUSH, HandEvaluator.evaluateHand(cards));
    }

    @Test
    public void testStraightFlush() {
        List<Card> cards = Arrays.asList(
                new Card(9, '♥'),
                new Card(8, '♥'),
                new Card(7, '♥'),
                new Card(6, '♥'),
                new Card(5, '♥'));

        assertEquals(HandRank.STRAIGHT_FLUSH, HandEvaluator.evaluateHand(cards));
    }

    @Test
    public void testFourOfAKind() {
        List<Card> cards = Arrays.asList(
                new Card(10, '♠'),
                new Card(10, '♥'),
                new Card(10, '♦'),
                new Card(10, '♣'),
                new Card(5, '♠'));

        assertEquals(HandRank.FOUR_OF_A_KIND, HandEvaluator.evaluateHand(cards));
    }

    @Test
    public void testFullHouse() {
        List<Card> cards = Arrays.asList(
                new Card(10, '♠'),
                new Card(10, '♥'),
                new Card(10, '♦'),
                new Card(5, '♠'),
                new Card(5, '♥'));

        assertEquals(HandRank.FULL_HOUSE, HandEvaluator.evaluateHand(cards));
    }

    @Test
    public void testFlush() {
        List<Card> cards = Arrays.asList(
                new Card(14, '♠'),
                new Card(11, '♠'),
                new Card(8, '♠'),
                new Card(5, '♠'),
                new Card(2, '♠'));

        assertEquals(HandRank.FLUSH, HandEvaluator.evaluateHand(cards));
    }

    @Test
    public void testStraight() {
        List<Card> cards = Arrays.asList(
                new Card(9, '♠'),
                new Card(8, '♥'),
                new Card(7, '♦'),
                new Card(6, '♣'),
                new Card(5, '♠'));

        assertEquals(HandRank.STRAIGHT, HandEvaluator.evaluateHand(cards));
    }

    @Test
    public void testWheelStraight() {
        // A-2-3-4-5 (roue)
        List<Card> cards = Arrays.asList(
                new Card(14, '♠'), // A
                new Card(2, '♥'),
                new Card(3, '♦'),
                new Card(4, '♣'),
                new Card(5, '♠'));

        assertEquals(HandRank.STRAIGHT, HandEvaluator.evaluateHand(cards));
    }

    @Test
    public void testThreeOfAKind() {
        List<Card> cards = Arrays.asList(
                new Card(9, '♠'),
                new Card(9, '♥'),
                new Card(9, '♦'),
                new Card(5, '♣'),
                new Card(2, '♠'));

        assertEquals(HandRank.THREE_OF_A_KIND, HandEvaluator.evaluateHand(cards));
    }

    @Test
    public void testTwoPair() {
        List<Card> cards = Arrays.asList(
                new Card(10, '♠'),
                new Card(10, '♥'),
                new Card(5, '♦'),
                new Card(5, '♣'),
                new Card(2, '♠'));

        assertEquals(HandRank.TWO_PAIR, HandEvaluator.evaluateHand(cards));
    }

    @Test
    public void testOnePair() {
        List<Card> cards = Arrays.asList(
                new Card(10, '♠'),
                new Card(10, '♥'),
                new Card(8, '♦'),
                new Card(5, '♣'),
                new Card(2, '♠'));

        assertEquals(HandRank.ONE_PAIR, HandEvaluator.evaluateHand(cards));
    }

    @Test
    public void testHighCard() {
        List<Card> cards = Arrays.asList(
                new Card(14, '♠'),
                new Card(11, '♥'),
                new Card(8, '♦'),
                new Card(5, '♣'),
                new Card(2, '♠'));

        assertEquals(HandRank.HIGH_CARD, HandEvaluator.evaluateHand(cards));
    }

    @Test
    public void testSevenCards() {
        // 7 cartes contenant une quinte flush cachée
        List<Card> cards = Arrays.asList(
                new Card(9, '♥'),
                new Card(8, '♥'),
                new Card(7, '♥'),
                new Card(6, '♥'),
                new Card(5, '♥'),
                new Card(2, '♠'),
                new Card(3, '♦'));

        assertEquals(HandRank.STRAIGHT_FLUSH, HandEvaluator.evaluateHand(cards));
    }

    @Test
    public void testHandStrength() {
        List<Card> royalFlush = Arrays.asList(
                new Card(14, '♠'), new Card(13, '♠'), new Card(12, '♠'),
                new Card(11, '♠'), new Card(10, '♠'));

        List<Card> highCard = Arrays.asList(
                new Card(7, '♠'), new Card(5, '♥'), new Card(3, '♦'),
                new Card(2, '♣'), new Card(14, '♠'));

        double royalStrength = HandEvaluator.getHandStrength(royalFlush);
        double highCardStrength = HandEvaluator.getHandStrength(highCard);

        assertTrue(royalStrength > highCardStrength);
        assertTrue(royalStrength > 0.9); // Royal Flush devrait être très forte
        assertTrue(highCardStrength < 0.3); // High Card devrait être faible
    }
}
