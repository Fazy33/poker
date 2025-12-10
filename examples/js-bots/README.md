# Bots JavaScript/TypeScript

Deux bots d'exemple pour s'affronter au poker:
- **bot_conservative.js** (JavaScript) : Stratégie prudente
- **bot_aggressive.ts** (TypeScript) : Stratégie agressive

## Installation

```bash
cd examples/js-bots
npm install
```

## Utilisation

### Option 1: Lancer les deux bots automatiquement

**Terminal 1** - Lancer le bot conservateur (qui crée la partie):
```bash
npm run bot-js ConservBot
```

Le bot affichera l'ID de la partie créée.

**Terminal 2** - Lancer le bot agressif (rejoindre la partie):
```bash
npm run bot-ts AggroBot <game_id>
```

**Terminal 3** - Démarrer la partie:
```bash
curl -X POST http://localhost:8080/api/games/<game_id>/start
```

### Option 2: Commandes individuelles

**Bot JavaScript (Conservateur)**:
```bash
node bot_conservative.js ConservBot
# ou pour rejoindre une partie existante:
node bot_conservative.js ConservBot <game_id>
```

**Bot TypeScript (Agressif)**:
```bash
npx ts-node bot_aggressive.ts AggroBot <game_id>
```

## Stratégies

### Bot Conservateur (JS)
- ✅ **Check** si possible
- ✅ **Call** seulement si la mise < 10% des jetons
- ❌ **Fold** sinon
- Jamais de raise

### Bot Agressif (TS)
- 🔥 **Raise** 60% du temps (15% des jetons)
- ✅ **Check** si raise impossible
- ✅ **Call** en dernier recours
- Rarement fold

## Observer la Partie

Ouvrez http://localhost:8080 dans votre navigateur pour voir les bots s'affronter en direct !

## Personnaliser

Modifiez la méthode `decideAction()` dans chaque bot pour créer votre propre stratégie.
