package com.poker.bot.api;

import com.google.gson.Gson;
import okhttp3.*;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

/**
 * Client pour communiquer avec l'API du serveur de poker
 */
public class PokerApiClient {

    private static final String API_BASE = "http://localhost:8080/api";
    private static final MediaType JSON = MediaType.get("application/json; charset=utf-8");

    private final OkHttpClient httpClient;
    private final Gson gson;
    private final String gameId;
    private String playerId;

    public PokerApiClient(String gameId) {
        this.gameId = gameId;
        this.httpClient = new OkHttpClient();
        this.gson = new Gson();
    }

    /**
     * Rejoint une partie de poker
     */
    public boolean joinGame(String botName) {
        Map<String, String> requestBody = new HashMap<>();
        requestBody.put("bot_name", botName);

        String json = gson.toJson(requestBody);
        RequestBody body = RequestBody.create(json, JSON);

        Request request = new Request.Builder()
                .url(API_BASE + "/games/" + gameId + "/join")
                .post(body)
                .build();

        try (Response response = httpClient.newCall(request).execute()) {
            if (response.isSuccessful() && response.body() != null) {
                String responseBody = response.body().string();
                @SuppressWarnings("unchecked")
                Map<String, Object> result = gson.fromJson(responseBody, Map.class);
                this.playerId = (String) result.get("player_id");
                System.out.println("✅ Rejoint la partie en position " + result.get("position"));
                System.out.println("   Player ID: " + playerId);
                return true;
            } else {
                System.err.println("❌ Erreur join: " + response.code());
                return false;
            }
        } catch (IOException e) {
            System.err.println("❌ Erreur de connexion: " + e.getMessage());
            return false;
        }
    }

    /**
     * Récupère l'état actuel de la partie
     */
    public GameState getGameState() {
        if (playerId == null) {
            return null;
        }

        HttpUrl url = HttpUrl.parse(API_BASE + "/games/" + gameId + "/state")
                .newBuilder()
                .addQueryParameter("player_id", playerId)
                .build();

        Request request = new Request.Builder()
                .url(url)
                .get()
                .build();

        try (Response response = httpClient.newCall(request).execute()) {
            if (response.isSuccessful() && response.body() != null) {
                String responseBody = response.body().string();
                return gson.fromJson(responseBody, GameState.class);
            }
        } catch (IOException e) {
            // Erreur silencieuse pour le polling
        }

        return null;
    }

    /**
     * Soumet une action au serveur
     */
    public boolean submitAction(String actionType, Integer amount) {
        if (playerId == null) {
            return false;
        }

        Map<String, Object> action = new HashMap<>();
        action.put("type", actionType);
        if (amount != null) {
            action.put("amount", amount);
        }

        Map<String, Object> requestBody = new HashMap<>();
        requestBody.put("player_id", playerId);
        requestBody.put("action", action);

        String json = gson.toJson(requestBody);
        RequestBody body = RequestBody.create(json, JSON);

        Request request = new Request.Builder()
                .url(API_BASE + "/games/" + gameId + "/action")
                .post(body)
                .build();

        try (Response response = httpClient.newCall(request).execute()) {
            if (response.isSuccessful() && response.body() != null) {
                String responseBody = response.body().string();
                @SuppressWarnings("unchecked")
                Map<String, Object> result = gson.fromJson(responseBody, Map.class);
                Boolean success = (Boolean) result.get("success");

                if (Boolean.TRUE.equals(success)) {
                    return true;
                } else {
                    System.err.println("❌ Action refusée: " + result.get("error"));
                    return false;
                }
            }
        } catch (IOException e) {
            System.err.println("❌ Erreur action: " + e.getMessage());
        }

        return false;
    }

    public String getPlayerId() {
        return playerId;
    }
}
