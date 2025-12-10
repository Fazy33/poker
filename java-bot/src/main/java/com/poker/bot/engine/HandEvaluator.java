package com.poker.bot.engine;

import java.util.*;
import java.util.stream.Collectors;

/**
 * Évalue les mains de poker et détermine les rangs
 */
public class HandEvaluator {

    /**
     * Évalue une main de 5 à 7 cartes et retourne le meilleur rang
     */
    public static HandRank evaluateHand(List<Card> cards) {
        if (cards.size() < 5) {
            return HandRank.HIGH_CARD;
        }

        // Si on a 7 cartes, trouver la meilleure combinaison de 5
        List<Card> bestHand = cards.size() == 7 ? findBestFiveCards(cards) : cards;

        // Vérifier dans l'ordre (du plus fort au plus faible pour optimisation)
        if (isRoyalFlush(bestHand))
            return HandRank.ROYAL_FLUSH;
        if (isStraightFlush(bestHand))
            return HandRank.STRAIGHT_FLUSH;
        if (isFourOfAKind(bestHand))
            return HandRank.FOUR_OF_A_KIND;
        if (isFullHouse(bestHand))
            return HandRank.FULL_HOUSE;
        if (isFlush(bestHand))
            return HandRank.FLUSH;
        if (isStraight(bestHand))
            return HandRank.STRAIGHT;
        if (isThreeOfAKind(bestHand))
            return HandRank.THREE_OF_A_KIND;
        if (isTwoPair(bestHand))
            return HandRank.TWO_PAIR;
        if (isOnePair(bestHand))
            return HandRank.ONE_PAIR;

        return HandRank.HIGH_CARD;
    }

    /**
     * Calcule la force normalisée d'une main (0.0 à 1.0)
     */
    public static double getHandStrength(List<Card> cards) {
        HandRank rank = evaluateHand(cards);

        // Force de base selon le rang
        double baseStrength = rank.getValue() / 10.0;

        // Ajustement selon les cartes hautes
        if (cards.size() >= 5) {
            List<Card> sorted = new ArrayList<>(cards);
            sorted.sort(Comparator.reverseOrder());

            // Bonus pour les cartes hautes
            double highCardBonus = sorted.get(0).getRank() / 140.0; // Max 14/140 = 0.1
            return Math.min(1.0, baseStrength + highCardBonus);
        }

        return baseStrength;
    }

    /**
     * Trouve la meilleure main de 5 cartes parmi 7
     */
    private static List<Card> findBestFiveCards(List<Card> sevenCards) {
        List<Card> bestHand = null;
        HandRank bestRank = HandRank.HIGH_CARD;

        // Générer toutes les combinaisons de 5 cartes parmi 7
        List<List<Card>> combinations = generateCombinations(sevenCards, 5);

        for (List<Card> combo : combinations) {
            HandRank rank = evaluateHandOfFive(combo);
            if (rank.getValue() > bestRank.getValue()) {
                bestRank = rank;
                bestHand = combo;
            }
        }

        return bestHand != null ? bestHand : sevenCards.subList(0, 5);
    }

    /**
     * Évalue une main de exactement 5 cartes
     */
    private static HandRank evaluateHandOfFive(List<Card> cards) {
        if (isRoyalFlush(cards))
            return HandRank.ROYAL_FLUSH;
        if (isStraightFlush(cards))
            return HandRank.STRAIGHT_FLUSH;
        if (isFourOfAKind(cards))
            return HandRank.FOUR_OF_A_KIND;
        if (isFullHouse(cards))
            return HandRank.FULL_HOUSE;
        if (isFlush(cards))
            return HandRank.FLUSH;
        if (isStraight(cards))
            return HandRank.STRAIGHT;
        if (isThreeOfAKind(cards))
            return HandRank.THREE_OF_A_KIND;
        if (isTwoPair(cards))
            return HandRank.TWO_PAIR;
        if (isOnePair(cards))
            return HandRank.ONE_PAIR;
        return HandRank.HIGH_CARD;
    }

    // Vérifications spécifiques de mains

    private static boolean isRoyalFlush(List<Card> cards) {
        if (!isStraightFlush(cards))
            return false;
        List<Card> sorted = new ArrayList<>(cards);
        sorted.sort(Comparator.reverseOrder());
        return sorted.get(0).getRank() == 14; // As
    }

    private static boolean isStraightFlush(List<Card> cards) {
        return isFlush(cards) && isStraight(cards);
    }

    private static boolean isFourOfAKind(List<Card> cards) {
        Map<Integer, Long> rankCounts = getRankCounts(cards);
        return rankCounts.containsValue(4L);
    }

    private static boolean isFullHouse(List<Card> cards) {
        Map<Integer, Long> rankCounts = getRankCounts(cards);
        return rankCounts.containsValue(3L) && rankCounts.containsValue(2L);
    }

    private static boolean isFlush(List<Card> cards) {
        if (cards.size() < 5)
            return false;
        char firstSuit = cards.get(0).getSuit();
        return cards.stream().allMatch(c -> c.getSuit() == firstSuit);
    }

    private static boolean isStraight(List<Card> cards) {
        if (cards.size() < 5)
            return false;

        List<Integer> ranks = cards.stream()
                .map(Card::getRank)
                .sorted()
                .distinct()
                .collect(Collectors.toList());

        if (ranks.size() < 5)
            return false;

        // Vérifier suite normale
        for (int i = 0; i <= ranks.size() - 5; i++) {
            boolean isStraight = true;
            for (int j = 0; j < 4; j++) {
                if (ranks.get(i + j + 1) != ranks.get(i + j) + 1) {
                    isStraight = false;
                    break;
                }
            }
            if (isStraight)
                return true;
        }

        // Vérifier suite A-2-3-4-5 (roue)
        if (ranks.contains(14) && ranks.contains(2) && ranks.contains(3) &&
                ranks.contains(4) && ranks.contains(5)) {
            return true;
        }

        return false;
    }

    private static boolean isThreeOfAKind(List<Card> cards) {
        Map<Integer, Long> rankCounts = getRankCounts(cards);
        return rankCounts.containsValue(3L);
    }

    private static boolean isTwoPair(List<Card> cards) {
        Map<Integer, Long> rankCounts = getRankCounts(cards);
        long pairCount = rankCounts.values().stream().filter(count -> count == 2L).count();
        return pairCount >= 2;
    }

    private static boolean isOnePair(List<Card> cards) {
        Map<Integer, Long> rankCounts = getRankCounts(cards);
        return rankCounts.containsValue(2L);
    }

    /**
     * Compte les occurrences de chaque rang
     */
    private static Map<Integer, Long> getRankCounts(List<Card> cards) {
        return cards.stream()
                .collect(Collectors.groupingBy(Card::getRank, Collectors.counting()));
    }

    /**
     * Génère toutes les combinaisons de k éléments parmi n
     */
    private static List<List<Card>> generateCombinations(List<Card> cards, int k) {
        List<List<Card>> result = new ArrayList<>();
        generateCombinationsHelper(cards, k, 0, new ArrayList<>(), result);
        return result;
    }

    private static void generateCombinationsHelper(List<Card> cards, int k, int start,
            List<Card> current, List<List<Card>> result) {
        if (current.size() == k) {
            result.add(new ArrayList<>(current));
            return;
        }

        for (int i = start; i < cards.size(); i++) {
            current.add(cards.get(i));
            generateCombinationsHelper(cards, k, i + 1, current, result);
            current.remove(current.size() - 1);
        }
    }
}
