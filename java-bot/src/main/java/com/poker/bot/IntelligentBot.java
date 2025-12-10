package com.poker.bot;

import com.poker.bot.api.GameState;
import com.poker.bot.api.PokerApiClient;
import com.poker.bot.engine.Card;
import com.poker.bot.engine.HandEvaluator;
import com.poker.bot.engine.HandRank;
import com.poker.bot.engine.ProbabilityCalculator;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;
import java.util.stream.Collectors;

/**
 * Bot de poker intelligent avec évaluation des mains et calcul de probabilités
 */
public class IntelligentBot {

    private final String name;
    private final PokerApiClient apiClient;
    private final Random random;

    public IntelligentBot(String name, String gameId) {
        this.name = name;
        this.apiClient = new PokerApiClient(gameId);
        this.random = new Random();
    }

    /**
     * Rejoint la partie
     */
    public boolean join() {
        return apiClient.joinGame(name);
    }

    /**
     * Lance la boucle de jeu principale
     */
    public void play() {
        System.out.println("\n🤖 Bot INTELLIGENT " + name + " en action!");
        System.out.println("   Stratégie: Décisions basées sur les probabilités et l'évaluation des mains\n");

        while (true) {
            try {
                GameState state = apiClient.getGameState();

                if (state == null) {
                    Thread.sleep(2000);
                    continue;
                }

                // Afficher l'état
                displayGameInfo(state);

                // Si c'est notre tour, décider et jouer
                if (state.isMyTurn()) {
                    System.out.println("\n🎯 C'est notre tour!");

                    Decision decision = makeDecision(state);
                    System.out.println("   → Décision: " + decision.toString());

                    apiClient.submitAction(decision.actionType, decision.amount);
                    Thread.sleep(1000);
                } else {
                    System.out.println("⏳ En attente...");
                }

                Thread.sleep(2000);

            } catch (InterruptedException e) {
                System.out.println("\n👋 Bot arrêté");
                break;
            } catch (Exception e) {
                System.err.println("❌ Erreur: " + e.getMessage());
                try {
                    Thread.sleep(5000);
                } catch (InterruptedException ie) {
                    break;
                }
            }
        }
    }

    /**
     * Prend une décision intelligente basée sur l'état du jeu
     */
    private Decision makeDecision(GameState state) {
        List<String> validActions = state.getValidActions();

        if (validActions == null || validActions.isEmpty()) {
            return new Decision("fold", null, "Aucune action valide");
        }

        // Parser les cartes
        List<Card> holeCards = parseCards(state.getYourCards());
        List<Card> communityCards = parseCards(state.getCommunityCards());

        // Calculer la force de la main et les probabilités
        double handStrength;
        double winProbability;
        HandRank currentRank = null;

        if (communityCards.isEmpty()) {
            // Pré-flop: utiliser l'évaluation pré-flop
            handStrength = ProbabilityCalculator.evaluatePreFlopStrength(holeCards);
            winProbability = handStrength;
        } else {
            // Post-flop: évaluer la main actuelle et simuler
            List<Card> allCards = new ArrayList<>(holeCards);
            allCards.addAll(communityCards);

            currentRank = HandEvaluator.evaluateHand(allCards);
            handStrength = HandEvaluator.getHandStrength(allCards);

            // Estimer le nombre d'adversaires actifs
            int activeOpponents = countActiveOpponents(state);

            // Monte Carlo simulation (adaptatif selon la phase)
            int simulations = getSimulationCount(state.getPhase());
            winProbability = ProbabilityCalculator.calculateWinProbability(
                    holeCards, communityCards, activeOpponents, simulations);
        }

        // Afficher les statistiques
        System.out.println("\n📊 Analyse:");
        if (currentRank != null) {
            System.out.println("   Main actuelle: " + currentRank);
        }
        System.out.println("   Force de la main: " + String.format("%.1f%%", handStrength * 100));
        System.out.println("   Probabilité de victoire: " + String.format("%.1f%%", winProbability * 100));
        System.out.println("   Actions valides: " + String.join(", ", validActions));

        // Calculer les pot odds si on doit call
        int amountToCall = state.getCurrentBet();
        double potOdds = 0;
        if (amountToCall > 0) {
            potOdds = ProbabilityCalculator.calculatePotOdds(state.getPot(), amountToCall);
            System.out.println("   Pot odds: " + String.format("%.1f%%", potOdds * 100));
            System.out.println("   Call profitable: " +
                    (ProbabilityCalculator.isCallProfitable(winProbability, potOdds) ? "OUI" : "NON"));
        }

        // Stratégie de décision
        return determineAction(state, validActions, winProbability, handStrength, potOdds);
    }

    /**
     * Détermine l'action à prendre basée sur les probabilités
     */
    private Decision determineAction(GameState state, List<String> validActions,
            double winProbability, double handStrength, double potOdds) {

        int chips = state.getYourChips();
        int pot = state.getPot();
        int currentBet = state.getCurrentBet();
        // Définition plus stricte de short stack: < 15% du pot OU < 100 jetons
        boolean isShortStack = chips < Math.max(pot * 0.15, 100);

        // Très forte main (> 80%)
        if (winProbability > 0.80) {
            if (validActions.contains("raise")) {
                int raiseAmount = calculateRaiseAmount(state, 0.3, 0.5); // 30-50% du pot
                return new Decision("raise", raiseAmount,
                        "Main très forte (" + String.format("%.0f%%", winProbability * 100) + ")");
            }
            if (validActions.contains("call")) {
                return new Decision("call", null, "Main très forte mais impossible de raise");
            }
            if (validActions.contains("check")) {
                // Slow play parfois avec des mains monstres
                if (random.nextDouble() < 0.3 && pot > 0) {
                    return new Decision("check", null, "Slow play avec main monstre");
                }
                return new Decision("check", null, "Main très forte");
            }
        }

        // Forte main (60-80%)
        if (winProbability > 0.60) {
            if (validActions.contains("raise")) {
                int raiseAmount = calculateRaiseAmount(state, 0.2, 0.4); // 20-40% du pot
                return new Decision("raise", raiseAmount,
                        "Main forte (" + String.format("%.0f%%", winProbability * 100) + ")");
            }
            if (validActions.contains("call")) {
                return new Decision("call", null, "Main forte");
            }
            if (validActions.contains("check")) {
                return new Decision("check", null, "Main forte");
            }
        }

        // Main moyenne (40-60%)
        if (winProbability > 0.40) {
            // Check si possible
            if (validActions.contains("check")) {
                return new Decision("check", null, "Main moyenne, pas de mise");
            }

            // Call si les pot odds sont bons
            if (validActions.contains("call")) {
                if (ProbabilityCalculator.isCallProfitable(winProbability, potOdds)) {
                    return new Decision("call", null,
                            "Main moyenne avec pot odds favorables");
                } else if (currentBet < chips * 0.1) {
                    // Call petit investissement
                    return new Decision("call", null, "Petit investissement");
                }
            }

            // Bluff léger occasionnellement
            if (validActions.contains("raise") && random.nextDouble() < 0.15) {
                int raiseAmount = calculateRaiseAmount(state, 0.15, 0.25);
                return new Decision("raise", raiseAmount, "Bluff semi-léger");
            }
        }

        // Main faible (20-40%)
        if (winProbability > 0.20) {
            if (validActions.contains("check")) {
                return new Decision("check", null, "Main faible, check gratuit");
            }

            // Call seulement avec très bons pot odds ou petit montant
            if (validActions.contains("call")) {
                if (potOdds < 0.15 || currentBet < chips * 0.05) {
                    return new Decision("call", null, "Pot odds exceptionnels ou très petit call");
                }
            }
        }

        // Main très faible (< 20%) - presque toujours fold
        // All-in de survie SEULEMENT si vraiment desperé (très peu de jetons)
        // ET avec une main au moins raisonnable
        if (isShortStack && validActions.contains("allin")) {
            // Conditions STRICTES pour all-in de survie:
            // - Moins de 2x la big blind approximativement (pot / 15)
            // - ET probabilité de victoire > 40% OU très bonne main (> 60%)
            boolean desperateMode = chips < pot / 15;
            if (desperateMode && (winProbability > 0.40 || handStrength > 0.60)) {
                return new Decision("allin", null,
                        "All-in désespéré (" + chips + " jetons, " +
                                String.format("%.0f%%", winProbability * 100) + " win prob)");
            }
        }

        // Check gratuit si possible
        if (validActions.contains("check")) {
            return new Decision("check", null, "Check par défaut");
        }

        // Sinon fold
        return new Decision("fold", null,
                "Main trop faible (" + String.format("%.0f%%", winProbability * 100) + ")");
    }

    /**
     * Calcule le montant de la relance
     */
    private int calculateRaiseAmount(GameState state, double minPotRatio, double maxPotRatio) {
        int pot = state.getPot();
        int currentBet = state.getCurrentBet();
        int chips = state.getYourChips();

        // Ratio aléatoire entre min et max
        double ratio = minPotRatio + random.nextDouble() * (maxPotRatio - minPotRatio);
        int raiseAmount = currentBet + (int) (pot * ratio);

        // S'assurer que c'est au moins la mise minimale
        int minRaise = currentBet + 20;
        raiseAmount = Math.max(raiseAmount, minRaise);

        // Ne pas dépasser nos jetons
        raiseAmount = Math.min(raiseAmount, chips);

        return raiseAmount;
    }

    /**
     * Compte le nombre d'adversaires actifs
     */
    private int countActiveOpponents(GameState state) {
        if (state.getPlayers() == null) {
            return 1; // Par défaut
        }

        return (int) state.getPlayers().stream()
                .filter(p -> !p.getId().equals(apiClient.getPlayerId()))
                .filter(p -> !"Folded".equals(p.getStatus()))
                .count();
    }

    /**
     * Retourne le nombre de simulations selon la phase
     */
    private int getSimulationCount(String phase) {
        switch (phase.toLowerCase()) {
            case "flop":
                return 2000; // Plus de cartes à venir, plus de simulations
            case "turn":
                return 3000;
            case "river":
                return 5000; // Dernière carte, simulations précises
            default:
                return 1000;
        }
    }

    /**
     * Parse une liste de cartes depuis le format API
     */
    private List<Card> parseCards(List<String> cardStrings) {
        if (cardStrings == null) {
            return new ArrayList<>();
        }

        return cardStrings.stream()
                .map(Card::fromString)
                .collect(Collectors.toList());
    }

    /**
     * Affiche les informations du jeu
     */
    private void displayGameInfo(GameState state) {
        System.out.println("\n📊 État du jeu:");
        System.out.println("   Phase: " + state.getPhase());
        System.out.println("   Pot: " + state.getPot());
        System.out.println("   Mise actuelle: " + state.getCurrentBet());
        System.out.println("   Vos jetons: " + state.getYourChips());

        if (state.getYourCards() != null && !state.getYourCards().isEmpty()) {
            System.out.println("   Vos cartes: " + String.join(", ", state.getYourCards()));
        }

        if (state.getCommunityCards() != null && !state.getCommunityCards().isEmpty()) {
            System.out.println("   Cartes communes: " + String.join(", ", state.getCommunityCards()));
        }
    }

    /**
     * Classe interne pour représenter une décision
     */
    private static class Decision {
        String actionType;
        Integer amount;
        String reason;

        Decision(String actionType, Integer amount, String reason) {
            this.actionType = actionType;
            this.amount = amount;
            this.reason = reason;
        }

        @Override
        public String toString() {
            String action = actionType.toUpperCase();
            if (amount != null) {
                action += " (" + amount + ")";
            }
            action += " - " + reason;
            return action;
        }
    }
}
