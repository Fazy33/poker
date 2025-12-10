#!/bin/bash

# Script pour arrêter tous les bots en cours

echo "🛑 Arrêt de tous les bots..."

# Arrêter tous les processus node bot_conservative.js
pkill -f "bot_conservative.js" && echo "  ✓ ConservBots arrêtés"

# Arrêter tous les processus bot_aggressive.ts
pkill -f "bot_aggressive.ts" && echo "  ✓ AggroBots arrêtés"

echo ""
echo "✅ Tous les bots ont été arrêtés"
