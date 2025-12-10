package com.poker.bot.engine;

import java.util.*;

/**
 * Calcule les probabilités de victoire et les pot odds
 */
public class ProbabilityCalculator {

    private static final Random random = new Random();

    /**
     * Évalue la force d'une main pré-flop (avant les cartes communes)
     */
    public static double evaluatePreFlopStrength(List<Card> holeCards) {
        if (holeCards.size() != 2) {
            return 0.5;
        }

        Card card1 = holeCards.get(0);
        Card card2 = holeCards.get(1);

        int rank1 = card1.getRank();
        int rank2 = card2.getRank();
        boolean suited = card1.getSuit() == card2.getSuit();
        boolean paired = rank1 == rank2;

        int highRank = Math.max(rank1, rank2);
        int lowRank = Math.min(rank1, rank2);

        // Paires premium
        if (paired) {
            if (rank1 >= 14)
                return 0.95; // AA
            if (rank1 >= 13)
                return 0.90; // KK
            if (rank1 >= 12)
                return 0.85; // QQ
            if (rank1 >= 11)
                return 0.80; // JJ
            if (rank1 >= 10)
                return 0.75; // TT
            if (rank1 >= 8)
                return 0.70; // 88-99
            if (rank1 >= 6)
                return 0.60; // 66-77
            return 0.50; // Petites paires
        }

        // Mains hautes assorties
        if (suited) {
            if (highRank == 14 && lowRank >= 13)
                return 0.85; // AKs
            if (highRank == 14 && lowRank >= 12)
                return 0.80; // AQs
            if (highRank == 14 && lowRank >= 11)
                return 0.75; // AJs
            if (highRank == 14 && lowRank >= 10)
                return 0.70; // ATs
            if (highRank == 13 && lowRank >= 12)
                return 0.75; // KQs
            if (highRank >= 11 && lowRank >= 10)
                return 0.65; // Connecteurs hauts assortis
        }

        // Mains hautes non assorties
        if (highRank == 14 && lowRank >= 13)
            return 0.80; // AK
        if (highRank == 14 && lowRank >= 12)
            return 0.75; // AQ
        if (highRank == 14 && lowRank >= 11)
            return 0.70; // AJ
        if (highRank == 13 && lowRank >= 12)
            return 0.70; // KQ

        // Connecteurs assortis
        if (suited && Math.abs(rank1 - rank2) <= 2) {
            return 0.55 + (highRank - 7) * 0.02;
        }

        // Par défaut, basé sur les cartes hautes
        double baseStrength = 0.3 + (highRank / 28.0) + (lowRank / 56.0);
        return Math.min(0.65, baseStrength);
    }

    /**
     * Simule des scénarios pour estimer la probabilité de victoire (Monte Carlo)
     * 
     * @param holeCards      Nos 2 cartes privées
     * @param communityCards Les cartes communes (0 à 5)
     * @param numOpponents   Nombre d'adversaires
     * @param simulations    Nombre de simulations à effectuer
     */
    public static double calculateWinProbability(List<Card> holeCards,
            List<Card> communityCards,
            int numOpponents,
            int simulations) {
        if (holeCards.size() != 2) {
            return 0.5;
        }

        int wins = 0;
        int ties = 0;

        // Créer le deck sans nos cartes et les cartes communes
        List<Card> usedCards = new ArrayList<>(holeCards);
        usedCards.addAll(communityCards);
        List<Card> deck = createDeck();
        deck.removeAll(usedCards);

        for (int i = 0; i < simulations; i++) {
            // Compléter les cartes communes si nécessaire
            Collections.shuffle(deck);
            List<Card> fullCommunity = new ArrayList<>(communityCards);
            int cardsNeeded = 5 - communityCards.size();
            for (int j = 0; j < cardsNeeded; j++) {
                fullCommunity.add(deck.get(j));
            }

            // Notre main finale
            List<Card> ourHand = new ArrayList<>(holeCards);
            ourHand.addAll(fullCommunity);
            HandRank ourRank = HandEvaluator.evaluateHand(ourHand);
            double ourStrength = HandEvaluator.getHandStrength(ourHand);

            // Simuler les mains des adversaires
            boolean weWin = true;
            boolean tied = false;

            for (int opp = 0; opp < numOpponents; opp++) {
                // Donner 2 cartes aléatoires à l'adversaire
                int cardIndex = cardsNeeded + opp * 2;
                if (cardIndex + 1 >= deck.size())
                    break;

                List<Card> oppHoleCards = Arrays.asList(
                        deck.get(cardIndex),
                        deck.get(cardIndex + 1));

                List<Card> oppHand = new ArrayList<>(oppHoleCards);
                oppHand.addAll(fullCommunity);
                HandRank oppRank = HandEvaluator.evaluateHand(oppHand);
                double oppStrength = HandEvaluator.getHandStrength(oppHand);

                if (oppRank.getValue() > ourRank.getValue() ||
                        (oppRank.getValue() == ourRank.getValue() && oppStrength > ourStrength)) {
                    weWin = false;
                    break;
                } else if (oppRank.getValue() == ourRank.getValue() &&
                        Math.abs(oppStrength - ourStrength) < 0.001) {
                    tied = true;
                }
            }

            if (weWin) {
                if (tied) {
                    ties++;
                } else {
                    wins++;
                }
            }
        }

        // Les ties comptent comme 0.5 win
        return (wins + ties * 0.5) / simulations;
    }

    /**
     * Calcule les pot odds (ratio entre le montant à call et le pot total)
     */
    public static double calculatePotOdds(int potSize, int amountToCall) {
        if (amountToCall == 0)
            return 1.0;
        return (double) amountToCall / (potSize + amountToCall);
    }

    /**
     * Détermine si un call est profitable basé sur les pot odds et l'equity
     */
    public static boolean isCallProfitable(double winProbability, double potOdds) {
        return winProbability > potOdds;
    }

    /**
     * Compte le nombre d'outs (cartes qui améliorent notre main)
     * Simplifié - retourne une estimation basée sur la main actuelle
     */
    public static int estimateOuts(List<Card> holeCards, List<Card> communityCards) {
        if (communityCards.isEmpty()) {
            // Pré-flop: estimation basée sur les hole cards
            Card c1 = holeCards.get(0);
            Card c2 = holeCards.get(1);

            if (c1.getRank() == c2.getRank()) {
                // Paire: ~2 outs pour un brelan
                return 2;
            }

            if (c1.getSuit() == c2.getSuit()) {
                // Cartes assorties: ~9 outs potentiels pour une couleur
                return 9;
            }

            // Cartes connectées: ~8 outs pour une suite
            if (Math.abs(c1.getRank() - c2.getRank()) <= 4) {
                return 8;
            }

            return 6; // Par défaut
        }

        // Post-flop: analyse plus détaillée
        List<Card> allCards = new ArrayList<>(holeCards);
        allCards.addAll(communityCards);

        HandRank currentRank = HandEvaluator.evaluateHand(allCards);

        // Estimation simple basée sur le rang actuel
        switch (currentRank) {
            case HIGH_CARD:
                return 6; // Cherche paire ou mieux
            case ONE_PAIR:
                return 5; // Cherche double paire ou brelan
            case TWO_PAIR:
                return 4; // Cherche full
            case THREE_OF_A_KIND:
                return 7; // Cherche full ou carré
            case STRAIGHT:
            case FLUSH:
                return 10; // Déjà une bonne main
            default:
                return 15; // Très bonne main
        }
    }

    /**
     * Crée un deck complet de 52 cartes
     */
    private static List<Card> createDeck() {
        List<Card> deck = new ArrayList<>();
        char[] suits = { '♠', '♥', '♦', '♣' };

        for (char suit : suits) {
            for (int rank = 2; rank <= 14; rank++) {
                deck.add(new Card(rank, suit));
            }
        }

        return deck;
    }
}
