# API Documentation - 4SH Poker Server

Documentation complète de l'API REST pour connecter des bots au serveur de poker.

## Base URL

```
http://localhost:8080/api
```

## Endpoints

### 1. Créer une Partie

Crée une nouvelle partie de poker.

**Endpoint:** `POST /api/games`

**Request Body:**
```json
{
  "name": "Ma Partie",
  "max_players": 6,
  "starting_chips": 1000,
  "small_blind": 10,
  "big_blind": 20
}
```

**Response:** `200 OK`
```json
{
  "game_id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Ma Partie"
}
```

**Exemple cURL:**
```bash
curl -X POST http://localhost:8080/api/games \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Game",
    "max_players": 4,
    "starting_chips": 1000,
    "small_blind": 10,
    "big_blind": 20
  }'
```

---

### 2. Lister les Parties

Récupère la liste de toutes les parties actives.

**Endpoint:** `GET /api/games`

**Response:** `200 OK`
```json
{
  "games": [
    {
      "game_id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "Ma Partie",
      "player_count": 2,
      "max_players": 6,
      "phase": "preflop",
      "pot": 30
    }
  ]
}
```

---

### 3. Rejoindre une Partie

Enregistre un bot dans une partie.

**Endpoint:** `POST /api/games/{game_id}/join`

**Request Body:**
```json
{
  "bot_name": "MonSuperBot",
  "bot_secret": "optional-auth-token"
}
```

**Response:** `200 OK`
```json
{
  "player_id": "MonSuperBot_uuid",
  "game_id": "550e8400-e29b-41d4-a716-446655440000",
  "position": 0
}
```

**Important:** Conservez le `player_id` retourné, il sera nécessaire pour toutes les actions futures.

**Exemple cURL:**
```bash
curl -X POST http://localhost:8080/api/games/{game_id}/join \
  -H "Content-Type: application/json" \
  -d '{"bot_name": "MyBot"}'
```

---

### 4. Démarrer une Partie

Démarre une partie (nécessite au moins 2 joueurs).

**Endpoint:** `POST /api/games/{game_id}/start`

**Response:** `200 OK`
```json
{
  "success": true
}
```

**Exemple cURL:**
```bash
curl -X POST http://localhost:8080/api/games/{game_id}/start
```

---

### 5. Obtenir l'État du Jeu

Récupère l'état actuel de la partie pour un joueur spécifique.

**Endpoint:** `GET /api/games/{game_id}/state?player_id={player_id}`

**Query Parameters:**
- `player_id` (required): L'ID du joueur obtenu lors du join

**Response:** `200 OK`
```json
{
  "game_id": "550e8400-e29b-41d4-a716-446655440000",
  "phase": "flop",
  "pot": 150,
  "current_bet": 50,
  "community_cards": ["A♠", "K♠", "Q♥"],
  "players": [
    {
      "id": "Bot1_uuid",
      "name": "Bot1",
      "chips": 950,
      "current_bet": 50,
      "status": "Active",
      "cards": null
    },
    {
      "id": "Bot2_uuid",
      "name": "Bot2",
      "chips": 900,
      "current_bet": 50,
      "status": "Active",
      "cards": null
    }
  ],
  "current_player_id": "Bot1_uuid",
  "your_player_id": "Bot1_uuid",
  "your_chips": 950,
  "your_cards": ["A♠", "K♦"],
  "valid_actions": ["fold", "check", "raise"]
}
```

**Champs importants:**
- `your_cards`: Vos 2 cartes privées (visibles uniquement par vous)
- `community_cards`: Les cartes communes sur la table
- `current_player_id`: L'ID du joueur dont c'est le tour
- `valid_actions`: Les actions que vous pouvez effectuer
- `your_chips`: Vos jetons restants

**Exemple cURL:**
```bash
curl "http://localhost:8080/api/games/{game_id}/state?player_id={player_id}"
```

---

### 6. Soumettre une Action

Effectue une action de jeu (fold, check, call, raise, allin).

**Endpoint:** `POST /api/games/{game_id}/action`

**Request Body:**
```json
{
  "player_id": "MonSuperBot_uuid",
  "action": {
    "type": "raise",
    "amount": 50
  }
}
```

**Types d'actions:**

1. **Fold** (se coucher)
```json
{
  "player_id": "...",
  "action": { "type": "fold" }
}
```

2. **Check** (checker - seulement si current_bet == votre mise)
```json
{
  "player_id": "...",
  "action": { "type": "check" }
}
```

3. **Call** (suivre)
```json
{
  "player_id": "...",
  "action": { "type": "call" }
}
```

4. **Raise** (relancer)
```json
{
  "player_id": "...",
  "action": {
    "type": "raise",
    "amount": 50
  }
}
```

5. **All-In** (tapis)
```json
{
  "player_id": "...",
  "action": { "type": "allin" }
}
```

**Response:** `200 OK`
```json
{
  "success": true,
  "error": null
}
```

**En cas d'erreur:**
```json
{
  "success": false,
  "error": "Ce n'est pas le tour de ce joueur"
}
```

**Exemple cURL:**
```bash
curl -X POST http://localhost:8080/api/games/{game_id}/action \
  -H "Content-Type: application/json" \
  -d '{
    "player_id": "MonBot_uuid",
    "action": {
      "type": "raise",
      "amount": 50
    }
  }'
```

---

## Flux de Jeu Typique

1. **Créer ou rejoindre une partie**
   ```
   POST /api/games (créer)
   POST /api/games/{id}/join (rejoindre)
   ```

2. **Attendre les autres joueurs**
   ```
   GET /api/games (vérifier le nombre de joueurs)
   ```

3. **Démarrer la partie** (quand assez de joueurs)
   ```
   POST /api/games/{id}/start
   ```

4. **Boucle de jeu:**
   ```
   a. GET /api/games/{id}/state (obtenir l'état)
   b. Si c'est votre tour (current_player_id == your_player_id):
      - Analyser votre main et l'état du jeu
      - Décider d'une action
      - POST /api/games/{id}/action (soumettre l'action)
   c. Attendre 1-2 secondes
   d. Répéter
   ```

---

## Codes d'Erreur

| Code | Signification |
|------|---------------|
| 200  | Succès |
| 400  | Requête invalide (vérifier le corps de la requête) |
| 404  | Partie non trouvée |
| 500  | Erreur serveur |

---

## Phases de Jeu

| Phase | Description |
|-------|-------------|
| `preflop` | Avant le flop (2 cartes privées distribuées) |
| `flop` | Après le flop (3 cartes communes) |
| `turn` | Après le turn (4ème carte commune) |
| `river` | Après la river (5ème carte commune) |
| `showdown` | Dévoilement des cartes |

---

## Statuts des Joueurs

| Statut | Description |
|--------|-------------|
| `Active` | Joueur actif dans la main |
| `Folded` | Joueur couché |
| `AllIn` | Joueur all-in |
| `SittingOut` | Joueur absent |

---

## Exemple Complet (Python)

Voir le fichier `examples/bot_example.py` pour un exemple complet de bot en Python.

**Utilisation:**
```bash
# Créer une partie et rejoindre
python examples/bot_example.py MonBot

# Rejoindre une partie existante
python examples/bot_example.py MonBot <game_id>
```

---

## Conseils pour Développer un Bot

1. **Polling:** Interrogez l'API toutes les 1-2 secondes pour obtenir l'état du jeu
2. **Validation:** Vérifiez toujours `valid_actions` avant de soumettre une action
3. **Gestion d'erreurs:** Gérez les erreurs réseau et les réponses d'erreur de l'API
4. **Stratégie:** Implémentez votre logique de décision dans une fonction séparée
5. **Logs:** Loggez toutes les actions pour faciliter le débogage

---

## Support

Pour toute question ou problème, consultez la documentation du projet ou ouvrez une issue sur GitHub.
