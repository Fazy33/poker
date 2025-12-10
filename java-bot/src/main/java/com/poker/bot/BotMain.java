package com.poker.bot;

/**
 * Point d'entrée principal du bot de poker intelligent
 */
public class BotMain {

    public static void main(String[] args) {
        if (args.length < 2) {
            System.out.println("Usage: java -jar intelligent-bot-1.0.jar <nom_du_bot> <game_id>");
            System.out.println("\nExemple:");
            System.out.println("  java -jar intelligent-bot-1.0.jar SmartBot abc123");
            System.exit(1);
        }

        String botName = args[0];
        String gameId = args[1];

        System.out.println("╔════════════════════════════════════════╗");
        System.out.println("║   🤖  BOT DE POKER INTELLIGENT  🤖    ║");
        System.out.println("╚════════════════════════════════════════╝");
        System.out.println();
        System.out.println("Nom du bot: " + botName);
        System.out.println("Game ID: " + gameId);

        IntelligentBot bot = new IntelligentBot(botName, gameId);

        if (!bot.join()) {
            System.err.println("\n❌ Impossible de rejoindre la partie");
            System.exit(1);
        }

        System.out.println("\n⏳ En attente d'autres joueurs...");
        System.out.println("   Utilisez la commande suivante pour démarrer la partie:");
        System.out.println("   curl -X POST http://localhost:8080/api/games/" + gameId + "/start");
        System.out.println();

        // Lancer la boucle de jeu
        bot.play();
    }
}
