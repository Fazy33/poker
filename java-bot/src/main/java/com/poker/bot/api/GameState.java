package com.poker.bot.api;

import java.util.List;

/**
 * Représente l'état du jeu reçu de l'API
 */
public class GameState {
    private String game_id;
    private String phase;
    private int pot;
    private int current_bet;
    private List<String> community_cards;
    private String current_player_id;
    private String your_player_id;
    private int your_chips;
    private List<String> your_cards;
    private List<String> valid_actions;
    private List<PlayerInfo> players;

    // Getters et setters
    public String getGameId() {
        return game_id;
    }

    public void setGameId(String game_id) {
        this.game_id = game_id;
    }

    public String getPhase() {
        return phase;
    }

    public void setPhase(String phase) {
        this.phase = phase;
    }

    public int getPot() {
        return pot;
    }

    public void setPot(int pot) {
        this.pot = pot;
    }

    public int getCurrentBet() {
        return current_bet;
    }

    public void setCurrentBet(int current_bet) {
        this.current_bet = current_bet;
    }

    public List<String> getCommunityCards() {
        return community_cards;
    }

    public void setCommunityCards(List<String> community_cards) {
        this.community_cards = community_cards;
    }

    public String getCurrentPlayerId() {
        return current_player_id;
    }

    public void setCurrentPlayerId(String current_player_id) {
        this.current_player_id = current_player_id;
    }

    public String getYourPlayerId() {
        return your_player_id;
    }

    public void setYourPlayerId(String your_player_id) {
        this.your_player_id = your_player_id;
    }

    public int getYourChips() {
        return your_chips;
    }

    public void setYourChips(int your_chips) {
        this.your_chips = your_chips;
    }

    public List<String> getYourCards() {
        return your_cards;
    }

    public void setYourCards(List<String> your_cards) {
        this.your_cards = your_cards;
    }

    public List<String> getValidActions() {
        return valid_actions;
    }

    public void setValidActions(List<String> valid_actions) {
        this.valid_actions = valid_actions;
    }

    public List<PlayerInfo> getPlayers() {
        return players;
    }

    public void setPlayers(List<PlayerInfo> players) {
        this.players = players;
    }

    public boolean isMyTurn() {
        return current_player_id != null && current_player_id.equals(your_player_id);
    }

    /**
     * Classe interne pour les informations des joueurs
     */
    public static class PlayerInfo {
        private String id;
        private String name;
        private int chips;
        private int current_bet;
        private String status;

        public String getId() {
            return id;
        }

        public void setId(String id) {
            this.id = id;
        }

        public String getName() {
            return name;
        }

        public void setName(String name) {
            this.name = name;
        }

        public int getChips() {
            return chips;
        }

        public void setChips(int chips) {
            this.chips = chips;
        }

        public int getCurrentBet() {
            return current_bet;
        }

        public void setCurrentBet(int current_bet) {
            this.current_bet = current_bet;
        }

        public String getStatus() {
            return status;
        }

        public void setStatus(String status) {
            this.status = status;
        }
    }
}
