# Moteur de Poker en Rust

Un moteur de poker Texas Hold'em complet avec serveur web, API pour bots et interface live.

## 🚀 Lancement Rapide

### 1. Démarrer le Serveur

```bash
./run-server.sh
```

Le serveur sera accessible sur :
- **Interface Web** : [http://localhost:8080](http://localhost:8080)
- **API REST** : [http://localhost:8080/api](http://localhost:8080/api)

### 2. Connecter un Bot

Un exemple de bot en Python est fourni :

```bash
# Créer une partie et rejoindre
python3 examples/bot_example.py MonBot

# Rejoindre une partie existante
python3 examples/bot_example.py AutreBot <game_id>
```

### 3. Lancer la Démo du Moteur (sans serveur)

```bash
./run-demo.sh
```

## 📁 Structure du Projet

```
4sh-poker/
├── poker-engine/       # Moteur de jeu (règles, cartes, mains)
├── poker-server/       # Serveur Web (Actix) et API REST
├── poker-ui/          # Interface Web (HTML/CSS/JS)
├── examples/          # Exemples de bots (Python)
└── docs/              # Documentation API
```

## 📡 API pour Bots

L'API permet de connecter des bots écrits dans n'importe quel langage.

- **Documentation complète** : [docs/API.md](docs/API.md)
- **Endpoints principaux** :
  - `POST /api/games` : Créer une partie
  - `POST /api/games/{id}/join` : Rejoindre une partie
  - `GET /api/games/{id}/state` : Obtenir l'état du jeu
  - `POST /api/games/{id}/action` : Jouer (fold, call, raise)

## 🧪 Tests

```bash
# Tester le moteur
~/.cargo/bin/cargo test --package poker-engine

# Tester le serveur
~/.cargo/bin/cargo test --package poker-server
```

## 📋 Installation de Rust

Si nécessaire :
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## ✨ Fonctionnalités

- **Moteur complet** : Texas Hold'em, gestion du pot, side-pots (basique), tous les rangs de mains.
- **Serveur performant** : Écrit en Rust avec Actix-web.
- **Interface Live** : Visualisation en temps réel des parties.
- **Multi-langage** : Les bots peuvent être en Python, JS, Rust, Java, etc.

## 📝 Licence

MIT
