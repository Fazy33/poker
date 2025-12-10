# Guide de Démarrage Rapide - 4SH Poker

Ce guide vous permet de démarrer rapidement avec le serveur de poker et les bots.

## 🚀 Démarrage en 5 Minutes

### Étape 1 : Démarrer le serveur

```bash
./run-server.sh
```

**Attendez que le serveur affiche :**
```
🎮 Démarrage du serveur de poker...
🌐 Serveur disponible sur http://localhost:8080
📡 API disponible sur http://localhost:8080/api
🎯 Interface web sur http://localhost:8080
```

### Étape 2 : Ouvrir l'interface web

Ouvrez votre navigateur : [http://localhost:8080](http://localhost:8080)

Vous verrez l'interface de poker avec :
- La liste des parties
- Un bouton "Créer une Partie"

### Étape 3 : Créer une partie

1. Cliquez sur **"Créer une Partie"**
2. Configurez la partie (ou gardez les valeurs par défaut)
3. Cliquez sur **"Créer"**

**Notez le `game_id`** qui apparaît dans l'URL : `http://localhost:8080?game=<game_id>`

### Étape 4 : Lancer des bots

Ouvrez **2 nouveaux terminaux** et lancez un bot dans chacun :

**Terminal 2 :**
```bash
python3 examples/bot_example.py Bot1 <game_id>
```

**Terminal 3 :**
```bash
python3 examples/bot_example.py Bot2 <game_id>
```

Remplacez `<game_id>` par l'ID de votre partie.

### Étape 5 : Démarrer la partie

Une fois que vous avez au moins 2 bots, démarrez la partie :

**Option A : Via l'interface web**
- Ouvrez la console du navigateur (F12)
- Tapez : `startGame('<game_id>')`

**Option B : Via cURL**
```bash
curl -X POST http://localhost:8080/api/games/<game_id>/start
```

### Étape 6 : Observer le jeu

Retournez sur l'interface web et observez :
- Les cartes communes qui apparaissent
- Les joueurs qui jouent leurs tours
- Le pot qui augmente
- Les actions dans le log

Les bots joueront automatiquement selon leur stratégie !

## 🤖 Créer Votre Propre Bot

### Bot Python Simple

```python
import requests
import time

API_BASE = "http://localhost:8080/api"

# 1. Rejoindre une partie
response = requests.post(
    f"{API_BASE}/games/<game_id>/join",
    json={"bot_name": "MonBot"}
)
player_id = response.json()["player_id"]

# 2. Boucle de jeu
while True:
    # Obtenir l'état
    state = requests.get(
        f"{API_BASE}/games/<game_id>/state",
        params={"player_id": player_id}
    ).json()
    
    # Est-ce notre tour ?
    if state["current_player_id"] == player_id:
        # Décider d'une action
        if "check" in state["valid_actions"]:
            action = {"type": "check"}
        elif "call" in state["valid_actions"]:
            action = {"type": "call"}
        else:
            action = {"type": "fold"}
        
        # Jouer
        requests.post(
            f"{API_BASE}/games/<game_id>/action",
            json={
                "player_id": player_id,
                "action": action
            }
        )
    
    time.sleep(2)
```

### Bot JavaScript/Node.js

```javascript
const axios = require('axios');

const API_BASE = 'http://localhost:8080/api';
const gameId = '<game_id>';
let playerId;

async function join() {
    const response = await axios.post(
        `${API_BASE}/games/${gameId}/join`,
        { bot_name: 'JSBot' }
    );
    playerId = response.data.player_id;
}

async function play() {
    const state = await axios.get(
        `${API_BASE}/games/${gameId}/state`,
        { params: { player_id: playerId } }
    );
    
    if (state.data.current_player_id === playerId) {
        const actions = state.data.valid_actions;
        let action;
        
        if (actions.includes('check')) {
            action = { type: 'check' };
        } else if (actions.includes('call')) {
            action = { type: 'call' };
        } else {
            action = { type: 'fold' };
        }
        
        await axios.post(
            `${API_BASE}/games/${gameId}/action`,
            { player_id: playerId, action }
        );
    }
}

async function main() {
    await join();
    setInterval(play, 2000);
}

main();
```

## 📊 Stratégies de Bot

### Stratégie Conservative
- Toujours `check` si possible
- `call` seulement avec une paire ou mieux
- `fold` sinon

### Stratégie Agressive
- `raise` avec une bonne main (paire d'as, roi, etc.)
- `call` avec une main moyenne
- `fold` avec une mauvaise main

### Stratégie Aléatoire (démo)
- Utilise `random` pour décider
- Bon pour tester le système

## 🎯 Organiser un Tournoi

1. **Créez plusieurs parties** avec des configurations identiques
2. **Invitez les participants** à connecter leurs bots
3. **Lancez toutes les parties** en même temps
4. **Observez** via l'interface web
5. **Comptez les points** :
   - 1er : 10 points
   - 2ème : 5 points
   - 3ème : 2 points
   - Participation : 1 point

## 🐛 Dépannage

### Le serveur ne démarre pas
```bash
# Vérifier que le port 8080 est libre
lsof -i :8080

# Si occupé, tuer le processus
kill -9 <PID>
```

### Le bot ne se connecte pas
- Vérifiez que le serveur est démarré
- Vérifiez le `game_id`
- Vérifiez que Python 3 est installé : `python3 --version`

### L'interface ne se rafraîchit pas
- Appuyez sur F5 pour rafraîchir
- Vérifiez la console du navigateur (F12)

## 📚 Ressources

- **API Documentation** : [docs/API.md](API.md)
- **Code du bot Python** : [examples/bot_example.py](../examples/bot_example.py)
- **README** : [README.md](../README.md)

## 💡 Conseils

- **Testez localement** votre bot avant de participer à un tournoi
- **Loggez vos décisions** pour pouvoir déboguer
- **Gérez les erreurs réseau** (timeout, reconnexion)
- **Optimisez votre stratégie** en analysant les résultats

Bonne chance ! 🍀
