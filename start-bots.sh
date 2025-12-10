#!/bin/bash

# Script pour lancer 3 bots sur la première partie disponible

API_URL="http://localhost:8080/api"
BOTS_DIR="examples/js-bots"

echo "🤖 Recherche de la première partie disponible..."

# Récupérer la liste des parties
GAMES_JSON=$(curl -s "$API_URL/games")

# Extraire le premier game_id
GAME_ID=$(echo "$GAMES_JSON" | grep -o '"game_id":"[^"]*"' | head -1 | cut -d'"' -f4)

if [ -z "$GAME_ID" ]; then
    echo "❌ Aucune partie trouvée. Créez d'abord une partie via l'interface web."
    exit 1
fi

echo "✅ Partie trouvée: $GAME_ID"
echo ""
echo "🚀 Lancement des bots..."

# Se déplacer dans le dossier des bots
cd "$BOTS_DIR" || exit 1

# Lancer les bots en arrière-plan
echo "  → Lancement de ConservBot1..."
node bot_conservative.js ConservBot1 "$GAME_ID" > /dev/null 2>&1 &
BOT1_PID=$!

sleep 1

echo "  → Lancement de ConservBot2..."
node bot_conservative.js ConservBot2 "$GAME_ID" > /dev/null 2>&1 &
BOT2_PID=$!

sleep 1

echo "  → Lancement d'AggroBot..."
npx ts-node bot_aggressive.ts AggroBot "$GAME_ID" > /dev/null 2>&1 &
BOT3_PID=$!

echo ""
echo "✅ Bots lancés avec succès!"
echo "   ConservBot1 (PID: $BOT1_PID)"
echo "   ConservBot2 (PID: $BOT2_PID)"
echo "   AggroBot    (PID: $BOT3_PID)"
echo ""
echo "📊 Ouvrez http://localhost:8080 pour voir la partie en direct"
echo ""
echo "Pour arrêter les bots:"
echo "   kill $BOT1_PID $BOT2_PID $BOT3_PID"
echo ""
echo "Ou utilisez: ./stop-bots.sh"
