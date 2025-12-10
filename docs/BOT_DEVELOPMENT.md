# Guide de Développement d'un Bot de Poker

Guide complet pour créer votre propre bot de poker pour la plateforme 4SH Poker.

## 📚 Table des Matières

1. [Concepts de Base](#concepts-de-base)
2. [Structure d'un Bot](#structure-dun-bot)
3. [API et Endpoints](#api-et-endpoints)
4. [Implémenter les Décisions](#implémenter-les-décisions)
5. [Stratégies de Jeu](#stratégies-de-jeu)
6. [Exemples par Langage](#exemples-par-langage)
7. [Conseils et Bonnes Pratiques](#conseils-et-bonnes-pratiques)

---

## Concepts de Base

### Le Cycle de Vie d'un Bot

```
1. Connexion au serveur
2. Création ou Rejoindre une partie
3. Boucle de jeu:
   - Récupérer l'état du jeu
   - Analyser la situation
   - Décider d'une action
   - Soumettre l'action
   - Attendre le prochain tour
4. Fin de partie
```

### États du Jeu

Un bot doit gérer plusieurs informations :

- **Phase de jeu** : PreFlop, Flop, Turn, River, Showdown
- **Vos cartes** : 2 cartes privées
- **Cartes communes** : 0 à 5 cartes sur la table
- **Pot total** : Argent en jeu
- **Mise actuelle** : Montant à égaler pour rester dans la partie
- **Vos jetons** : Combien il vous reste
- **Actions valides** : Ce que vous pouvez faire maintenant

---

## Structure d'un Bot

### Architecture Recommandée

```
Bot
├── Connexion (HTTP client)
├── Gestion d'État
│   ├── Mémoriser le game_id
│   ├── Mémoriser le player_id
│   └── Suivre l'état actuel
├── Logique de Décision
│   ├── Analyser les cartes
│   ├── Évaluer la position
│   └── Choisir une action
└── Boucle de Jeu
    ├── Polling (toutes les 1-2s)
    └── Gestion d'erreurs
```

### Composants Essentiels

**1. Client API**
```javascript
class PokerBot {
    constructor(apiUrl, botName) {
        this.apiUrl = apiUrl;
        this.botName = botName;
        this.gameId = null;
        this.playerId = null;
    }
}
```

**2. Méthodes de Base**
- `joinGame(gameId)` : Rejoindre une partie
- `getGameState()` : Récupérer l'état actuel
- `submitAction(action)` : Jouer une action
- `decideAction(state)` : Logique de décision

**3. Boucle de Jeu**
```javascript
async play() {
    while (true) {
        const state = await this.getGameState();
        
        if (state.current_player_id === this.playerId) {
            const action = this.decideAction(state);
            await this.submitAction(action);
        }
        
        await sleep(2000); // Attendre 2 secondes
    }
}
```

---

## API et Endpoints

### 1. Rejoindre une Partie

```http
POST /api/games/{game_id}/join
Content-Type: application/json

{
    "bot_name": "MonBot"
}
```

**Réponse:**
```json
{
    "player_id": "MonBot_uuid",
    "game_id": "550e8400-...",
    "position": 0
}
```

### 2. Récupérer l'État du Jeu

```http
GET /api/games/{game_id}/state?player_id={player_id}
```

**Réponse:**
```json
{
    "phase": "flop",
    "pot": 150,
    "current_bet": 50,
    "your_chips": 950,
    "your_cards": ["A♠", "K♠"],
    "community_cards": ["Q♠", "J♠", "10♥"],
    "current_player_id": "MonBot_uuid",
    "valid_actions": ["fold", "call", "raise"]
}
```

### 3. Soumettre une Action

```http
POST /api/games/{game_id}/action
Content-Type: application/json

{
    "player_id": "MonBot_uuid",
    "action": {
        "type": "raise",
        "amount": 100
    }
}
```

**Types d'actions:**
- `"fold"` : Se coucher
- `"check"` : Checker (si current_bet == 0)
- `"call"` : Suivre la mise
- `"raise"` avec `amount` : Relancer
- `"allin"` : Tapis

---

## Implémenter les Décisions

### Patron de Décision de Base

```javascript
function decideAction(state) {
    const { valid_actions, your_cards, community_cards, 
            current_bet, your_chips, pot } = state;
    
    // 1. Évaluer votre main
    const handStrength = evaluateHand(your_cards, community_cards);
    
    // 2. Calculer les cotes
    const potOdds = current_bet / (pot + current_bet);
    
    // 3. Décider
    if (handStrength > 0.8) {
        return { type: 'raise', amount: pot * 0.5 };
    } else if (handStrength > 0.5 && valid_actions.includes('call')) {
        return { type: 'call' };
    } else if (valid_actions.includes('check')) {
        return { type: 'check' };
    } else {
        return { type: 'fold' };
    }
}
```

### Évaluer la Force de la Main

**Simple (Cartes Hautes):**
```javascript
function simpleHandStrength(cards) {
    // Chercher les hautes cartes
    const ranks = cards.map(c => c[0]); // 'A', 'K', 'Q', etc.
    
    if (ranks.includes('A')) return 0.9;
    if (ranks.includes('K')) return 0.7;
    if (ranks.includes('Q')) return 0.5;
    return 0.3;
}
```

**Avancé (Détection de Combinaisons):**
```javascript
function advancedHandStrength(myCards, communityCards) {
    const allCards = [...myCards, ...communityCards];
    
    // Détecter paires, couleurs, suites, etc.
    if (hasPair(allCards)) return 0.6;
    if (hasTwoPair(allCards)) return 0.7;
    if (hasThreeOfKind(allCards)) return 0.8;
    if (hasFlush(allCards)) return 0.85;
    if (hasStraight(allCards)) return 0.85;
    if (hasFullHouse(allCards)) return 0.9;
    
    return 0.3; // Carte haute
}
```

---

## Stratégies de Jeu

### 1. Stratégie Conservative (Tight-Passive)

**Principe:** Jouer peu de mains, mais bien.

```javascript
function conservativeStrategy(state) {
    const { valid_actions, your_chips, current_bet } = state;
    
    // Ne miser que si bon jeu
    if (valid_actions.includes('check')) {
        return { type: 'check' };
    }
    
    // Call seulement si petit montant
    const callCost = current_bet;
    if (callCost < your_chips * 0.1 && valid_actions.includes('call')) {
        return { type: 'call' };
    }
    
    return { type: 'fold' };
}
```

**Avantages:**
- ✅ Perte lente de jetons
- ✅ Survie longue
- ❌ Gains limités

### 2. Stratégie Agressive (Loose-Aggressive)

**Principe:** Jouer beaucoup de mains, miser gros.

```javascript
function aggressiveStrategy(state) {
    const { valid_actions, your_chips, pot } = state;
    
    // Raise souvent
    if (valid_actions.includes('raise') && Math.random() < 0.6) {
        const raiseAmount = Math.floor(pot * 0.75);
        return { type: 'raise', amount: raiseAmount };
    }
    
    if (valid_actions.includes('call')) {
        return { type: 'call' };
    }
    
    if (valid_actions.includes('check')) {
        return { type: 'check' };
    }
    
    return { type: 'fold' };
}
```

**Avantages:**
- ✅ Gains rapides possibles
- ✅ Intimide les adversaires
- ❌ Perte rapide de jetons si malchance

### 3. Stratégie Adaptative

**Principe:** Changer de style selon la situation.

```javascript
function adaptiveStrategy(state) {
    const { phase, your_chips, pot } = state;
    
    // Conservateur en début de partie
    if (your_chips > 800) {
        return conservativeStrategy(state);
    }
    
    // Agressif en fin de partie ou si petit stack
    if (your_chips < 300 || phase === 'river') {
        return aggressiveStrategy(state);
    }
    
    // Équilibré sinon
    return balancedStrategy(state);
}
```

---

## Exemples par Langage

### Python

```python
import requests
import time

class PokerBot:
    def __init__(self, api_url, bot_name):
        self.api_url = api_url
        self.bot_name = bot_name
        self.player_id = None
        
    def join_game(self, game_id):
        response = requests.post(
            f"{self.api_url}/games/{game_id}/join",
            json={"bot_name": self.bot_name}
        )
        self.player_id = response.json()["player_id"]
        
    def get_state(self, game_id):
        response = requests.get(
            f"{self.api_url}/games/{game_id}/state",
            params={"player_id": self.player_id}
        )
        return response.json()
        
    def submit_action(self, game_id, action):
        requests.post(
            f"{self.api_url}/games/{game_id}/action",
            json={
                "player_id": self.player_id,
                "action": action
            }
        )
        
    def decide_action(self, state):
        # Votre logique ici
        if "check" in state["valid_actions"]:
            return {"type": "check"}
        return {"type": "fold"}
```

### JavaScript/Node.js

```javascript
const axios = require('axios');

class PokerBot {
    constructor(apiUrl, botName) {
        this.apiUrl = apiUrl;
        this.botName = botName;
        this.playerId = null;
    }
    
    async joinGame(gameId) {
        const response = await axios.post(
            `${this.apiUrl}/games/${gameId}/join`,
            { bot_name: this.botName }
        );
        this.playerId = response.data.player_id;
    }
    
    async getState(gameId) {
        const response = await axios.get(
            `${this.apiUrl}/games/${gameId}/state`,
            { params: { player_id: this.playerId } }
        );
        return response.data;
    }
    
    async submitAction(gameId, action) {
        await axios.post(
            `${this.apiUrl}/games/${gameId}/action`,
            {
                player_id: this.playerId,
                action: action
            }
        );
    }
    
    decideAction(state) {
        // Votre logique ici
        if (state.valid_actions.includes('check')) {
            return { type: 'check' };
        }
        return { type: 'fold' };
    }
}
```

### Java

```java
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.net.URI;
import com.google.gson.Gson;

public class PokerBot {
    private final String apiUrl;
    private final String botName;
    private String playerId;
    private final HttpClient client;
    private final Gson gson;
    
    public PokerBot(String apiUrl, String botName) {
        this.apiUrl = apiUrl;
        this.botName = botName;
        this.client = HttpClient.newHttpClient();
        this.gson = new Gson();
    }
    
    public void joinGame(String gameId) throws Exception {
        String json = gson.toJson(Map.of("bot_name", botName));
        
        HttpRequest request = HttpRequest.newBuilder()
            .uri(URI.create(apiUrl + "/games/" + gameId + "/join"))
            .header("Content-Type", "application/json")
            .POST(HttpRequest.BodyPublishers.ofString(json))
            .build();
            
        HttpResponse<String> response = client.send(
            request,
            HttpResponse.BodyHandlers.ofString()
        );
        
        // Parser la réponse et extraire player_id
    }
    
    // Autres méthodes...
}
```

---

## Conseils et Bonnes Pratiques

### 🎯 Développement

1. **Commencez simple**
   - Stratégie basique d'abord (check/fold)
   - Ajoutez de la complexité progressivement

2. **Testez localement**
   - Lancez votre bot contre les bots d'exemple
   - Vérifiez qu'il ne crash pas

3. **Logs détaillés**
   ```javascript
   console.log(`[${new Date().toISOString()}] Action: ${action.type}`);
   ```

4. **Gestion d'erreurs robuste**
   ```javascript
   try {
       await submitAction(action);
   } catch (error) {
       console.error('Erreur:', error);
       // Réessayer ou action par défaut (fold)
   }
   ```

### 🚀 Performance

1. **Polling intelligent**
   - 1-2 secondes entre chaque requête
   - Ne pas spammer l'API

2. **Timeout des requêtes**
   ```javascript
   axios.get(url, { timeout: 5000 })
   ```

3. **Reconnexion automatique**
   ```javascript
   if (error.code === 'ECONNREFUSED') {
       await sleep(5000);
       continue; // Réessayer
   }
   ```

### 🧠 Stratégie

1. **Adaptez-vous à la phase**
   - PreFlop : Sélectif
   - Flop : Évaluer le potentiel
   - Turn/River : Plus agressif si bonne main

2. **Gérez votre stack**
   - Stack large (>800) : Conservateur
   - Stack moyen (300-800) : Équilibré  
   - Stack court (<300) : Agressif (all-in)

3. **Bluff calculé**
   ```javascript
   // 10% de chances de bluffer
   if (Math.random() < 0.1 && valid_actions.includes('raise')) {
       return { type: 'raise', amount: pot * 0.5 };
   }
   ```

### ⚠️ Pièges à Éviter

❌ **Ne pas gérer les erreurs réseau**
❌ **Prendre trop de temps pour décider (>5s)**
❌ **Ne pas respecter les `valid_actions`**
❌ **Miser plus que `your_chips`**
❌ **Oublier de vérifier `current_player_id`**

### ✅ Checklist Avant Tournoi

- [ ] Le bot se connecte correctement
- [ ] Le bot rejoint une partie
- [ ] Le bot joue toutes les actions valides
- [ ] Les erreurs sont gérées
- [ ] Les logs sont clairs
- [ ] Testé contre d'autres bots
- [ ] Performance OK (pas de lag)

---

## Ressources

- **API Documentation** : [docs/API.md](API.md)
- **Exemples de bots** :
  - Python : [examples/bot_example.py](../bot_example.py)
  - JavaScript : [examples/js-bots/bot_conservative.js](../js-bots/bot_conservative.js)
  - TypeScript : [examples/js-bots/bot_aggressive.ts](../js-bots/bot_aggressive.ts)

---

## Support

Pour toute question, consultez la documentation ou testez avec les bots d'exemple fournis.

**Bon code et que le meilleur bot gagne ! 🃏🤖**
