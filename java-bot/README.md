# Bot de Poker Intelligent en Java

Un bot de poker intelligent qui utilise l'évaluation des mains et le calcul de probabilités pour prendre des décisions optimales.

## 🎯 Fonctionnalités

- **Évaluation complète des mains** : Détecte tous les rangs de mains du poker (High Card → Royal Flush)
- **Calcul de probabilités** : Utilise des simulations Monte Carlo pour estimer les chances de victoire
- **Stratégie intelligente** : Prend des décisions basées sur :
  - Force de la main actuelle
  - Probabilité de victoire estimée
  - Pot odds (ratio risque/récompense)
  - Taille du stack
  - Position dans le jeu
- **Adaptabilité** : Ajuste sa stratégie selon la phase du jeu (pré-flop, flop, turn, river)

## 📋 Prérequis

- Java 11 ou supérieur
- Maven 3.6 ou supérieur

## 🔨 Compilation

```bash
cd java-bot
mvn clean package
```

Cette commande va :
1. Télécharger toutes les dépendances
2. Compiler le code source
3. Exécuter les tests
4. Créer un JAR exécutable dans `target/intelligent-bot-1.0.jar`

## 🚀 Utilisation

### 1. Démarrer le serveur de poker

Dans le répertoire racine du projet :

```bash
./run-server.sh
```

Le serveur sera accessible sur http://localhost:8080

### 2. Créer une partie

Ouvrez votre navigateur et allez sur http://localhost:8080, ou utilisez l'API :

```bash
curl -X POST http://localhost:8080/api/games \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Partie Test",
    "max_players": 4,
    "starting_chips": 1000,
    "small_blind": 10,
    "big_blind": 20
  }'
```

Notez le `game_id` retourné.

### 3. Lancer le bot Java

```bash
cd java-bot
java -jar target/intelligent-bot-1.0.jar SmartBot <game_id>
```

Ou utilisez le script de lancement :

```bash
./start-java-bot.sh SmartBot <game_id>
```

### 4. Lancer d'autres bots (optionnel)

Pour une partie complète, lancez d'autres bots :

```bash
# Bot conservateur (JavaScript)
node examples/js-bots/bot_conservative.js ConservBot <game_id>

# Bot agressif (TypeScript)
npx ts-node examples/js-bots/bot_aggressive.ts AggroBot <game_id>
```

### 5. Démarrer la partie

```bash
curl -X POST http://localhost:8080/api/games/<game_id>/start
```

## 🧠 Stratégie du Bot

Le bot analyse chaque situation et prend des décisions basées sur plusieurs facteurs :

### Évaluation Pré-Flop
- Main premium (AA, KK, QQ) → Très agressif
- Main forte (AK, AQ, JJ) → Agressif
- Main moyenne → Prudent
- Main faible → Fold ou call si pot odds favorables

### Évaluation Post-Flop
1. **Calcul de la force de la main** : Évalue la meilleure main de 5 cartes
2. **Simulation Monte Carlo** : 
   - Flop : 2000 simulations
   - Turn : 3000 simulations
   - River : 5000 simulations
3. **Analyse des pot odds** : Détermine si un call est mathématiquement rentable
4. **Décision finale** basée sur :
   - Probabilité de victoire > 80% → Raise agressif
   - Probabilité de victoire 60-80% → Raise modéré
   - Probabilité de victoire 40-60% → Check/Call prudent
   - Probabilité de victoire 20-40% → Check si gratuit, sinon fold
   - Probabilité de victoire < 20% → Fold (sauf short stack)

### Gestion du Stack
- **Short stack** (< 50% du pot) : Stratégie push/fold plus agressive
- **Medium stack** : Stratégie équilibrée
- **Deep stack** : Peut se permettre plus de spéculation

## 📊 Exemple de Sortie

```
🤖 Bot INTELLIGENT SmartBot en action!
   Stratégie: Décisions basées sur les probabilités et l'évaluation des mains

📊 État du jeu:
   Phase: flop
   Pot: 150
   Mise actuelle: 50
   Vos jetons: 900
   Vos cartes: A♠, K♦
   Cartes communes: A♥, 10♠, 7♣

🎯 C'est notre tour!

📊 Analyse:
   Main actuelle: Paire
   Force de la main: 68.5%
   Probabilité de victoire: 72.3%
   Actions valides: fold, call, raise
   Pot odds: 25.0%
   Call profitable: OUI
   → Décision: RAISE (80) - Main forte (72%)
```

## 🧪 Tests

Pour exécuter les tests unitaires :

```bash
cd java-bot
mvn test
```

Les tests vérifient :
- Parsing correct des cartes
- Évaluation correcte de toutes les mains de poker
- Calculs de probabilités cohérents

## 🏗️ Architecture

```
java-bot/
├── src/main/java/com/poker/bot/
│   ├── BotMain.java              # Point d'entrée
│   ├── IntelligentBot.java       # Logique principale du bot
│   ├── api/
│   │   ├── GameState.java        # Modèle de l'état du jeu
│   │   └── PokerApiClient.java   # Client HTTP pour l'API
│   └── engine/
│       ├── Card.java             # Représentation d'une carte
│       ├── HandRank.java         # Énumération des rangs
│       ├── HandEvaluator.java    # Évaluation des mains
│       └── ProbabilityCalculator.java  # Calculs de probabilités
└── pom.xml                       # Configuration Maven
```

## 🔧 Dépendances

- **Gson** : Sérialisation/désérialisation JSON
- **OkHttp** : Client HTTP pour communiquer avec l'API
- **JUnit** : Framework de tests

## 🎓 Apprendre du Bot

Le code est commenté et structuré de manière pédagogique. Vous pouvez :

1. **Modifier la stratégie** dans `IntelligentBot.determineAction()`
2. **Ajuster les seuils de probabilité** pour rendre le bot plus/moins agressif
3. **Améliorer l'évaluation pré-flop** dans `ProbabilityCalculator.evaluatePreFlopStrength()`
4. **Augmenter la précision** en ajustant le nombre de simulations Monte Carlo

## 📝 Licence

MIT
