# Guide d'Organisation de Tournoi

Ce guide vous aidera à organiser un concours de bots de poker en utilisant le serveur 4SH Poker.

## 🏆 Format du Tournoi

### 1. Structure Recommandée

Pour un tournoi équitable, nous recommandons une structure en **championnat** :

- **Phase de Qualification** : Plusieurs tables de 4-6 bots.
- **Système de Points** : Les bots gagnent des points selon leur classement à chaque partie.
- **Finale** : Les meilleurs bots s'affrontent sur une table finale.

### 2. Barème de Points

Exemple de barème pour une table de 6 joueurs :

| Position | Points |
|----------|--------|
| 1er      | 10 pts |
| 2ème     | 6 pts  |
| 3ème     | 4 pts  |
| 4ème     | 2 pts  |
| 5ème     | 1 pt   |
| 6ème     | 0 pt   |

## 🛠 Préparation Technique

### 1. Serveur Central

Désignez une machine "maître" qui fera tourner le serveur :
```bash
./run-server.sh
```
Assurez-vous que cette machine est accessible par tous les participants (réseau local ou IP publique).

### 2. Configuration des Parties

Créez des parties identiques pour chaque ronde :
- **Jetons de départ** : 1000 (standard) ou 10,000 (deep stack)
- **Blinds** : 10/20 (standard)
- **Joueurs max** : 6

### 3. Connexion des Participants

Fournissez aux participants :
- L'URL du serveur (ex: `http://192.168.1.x:8080`)
- L'ID de la partie (`game_id`) pour leur table

## 📝 Règles du Concours

1. **Temps de Réponse** : Les bots doivent jouer en moins de 2 secondes.
2. **Stabilité** : Un bot qui crash ou ne répond pas est considéré comme "Fold".
3. **Fair-play** : Interdiction de spammer l'API ou de tenter de faire crasher le serveur.

## 📊 Déroulement d'une Partie

1. **Lancement** : L'organisateur crée la partie et partage l'ID.
2. **Inscription** : Les participants lancent leurs bots qui rejoignent la partie.
3. **Vérification** : L'organisateur vérifie sur l'interface web que tous les bots sont présents.
4. **Start** : L'organisateur démarre la partie (via l'interface ou API).
5. **Jeu** : La partie se déroule automatiquement.
6. **Résultat** : Notez l'ordre d'élimination des bots.

## 💡 Conseils pour les Participants

- **Testez votre bot** localement avant le tournoi.
- **Gérez les erreurs** réseau (reconnexion automatique).
- **Loggez tout** pour pouvoir analyser vos parties après coup.
- **Prévoyez plusieurs stratégies** (agressive, passive) si votre bot le permet.

## 🏅 Exemple de Grille de Score

| Bot | Partie 1 | Partie 2 | Partie 3 | Total |
|-----|----------|----------|----------|-------|
| Bot A | 10 | 4 | 6 | **20** |
| Bot B | 6 | 10 | 2 | **18** |
| Bot C | 4 | 2 | 10 | **16** |
| Bot D | 2 | 6 | 4 | **12** |

Que le meilleur code gagne ! 🚀
