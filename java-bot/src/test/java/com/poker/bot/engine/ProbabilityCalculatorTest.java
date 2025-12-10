package com.poker.bot.engine;

import org.junit.Test;
import static org.junit.Assert.*;

import java.util.Arrays;
import java.util.List;

/**
 * Tests pour la classe ProbabilityCalculator
 */
public class ProbabilityCalculatorTest {

    @Test
    public void testPreFlopPocketAces() {
        List<Card> pocketAces = Arrays.asList(
                new Card(14, '♠'),
                new Card(14, '♥'));

        double strength = ProbabilityCalculator.evaluatePreFlopStrength(pocketAces);
        assertTrue("AA devrait avoir une force > 0.9", strength > 0.9);
    }

    @Test
    public void testPreFlopAceKingSuited() {
        List<Card> akSuited = Arrays.asList(
                new Card(14, '♠'),
                new Card(13, '♠'));

        double strength = ProbabilityCalculator.evaluatePreFlopStrength(akSuited);
        assertTrue("AKs devrait avoir une force > 0.8", strength > 0.8);
    }

    @Test
    public void testPreFlopWeakHand() {
        List<Card> weakHand = Arrays.asList(
                new Card(7, '♠'),
                new Card(2, '♥'));

        double strength = ProbabilityCalculator.evaluatePreFlopStrength(weakHand);
        assertTrue("7-2 devrait avoir une force < 0.65", strength < 0.65);
        assertTrue("7-2 devrait avoir une force > 0.3", strength > 0.3);
    }

    @Test
    public void testPotOdds() {
        // Pot de 100, besoin de call 25
        double potOdds = ProbabilityCalculator.calculatePotOdds(100, 25);
        assertEquals(0.20, potOdds, 0.01); // 25/(100+25) = 0.20
    }

    @Test
    public void testCallProfitability() {
        // Win probability 30%, pot odds 20% -> profitable
        assertTrue(ProbabilityCalculator.isCallProfitable(0.30, 0.20));

        // Win probability 15%, pot odds 20% -> non profitable
        assertFalse(ProbabilityCalculator.isCallProfitable(0.15, 0.20));
    }

    @Test
    public void testWinProbabilityWithStrongHand() {
        // Paire d'As post-flop
        List<Card> holeCards = Arrays.asList(
                new Card(14, '♠'),
                new Card(14, '♥'));

        List<Card> communityCards = Arrays.asList(
                new Card(7, '♦'),
                new Card(8, '♣'),
                new Card(2, '♠'));

        double winProb = ProbabilityCalculator.calculateWinProbability(
                holeCards, communityCards, 2, 500);

        assertTrue("Overpair devrait avoir une bonne probabilité de victoire",
                winProb > 0.5);
    }

    @Test
    public void testWinProbabilityMadeHand() {
        // On a une couleur
        List<Card> holeCards = Arrays.asList(
                new Card(14, '♠'),
                new Card(10, '♠'));

        List<Card> communityCards = Arrays.asList(
                new Card(7, '♠'),
                new Card(5, '♠'),
                new Card(2, '♠'));

        double winProb = ProbabilityCalculator.calculateWinProbability(
                holeCards, communityCards, 2, 500);

        assertTrue("Couleur au flop devrait avoir une très bonne probabilité",
                winProb > 0.7);
    }

    @Test
    public void testEstimateOutsWithPair() {
        List<Card> holeCards = Arrays.asList(
                new Card(10, '♠'),
                new Card(10, '♥'));

        List<Card> communityCards = Arrays.asList(
                new Card(7, '♦'),
                new Card(8, '♣'),
                new Card(2, '♠'));

        int outs = ProbabilityCalculator.estimateOuts(holeCards, communityCards);
        assertTrue("Une paire devrait avoir quelques outs", outs >= 2);
    }
}
