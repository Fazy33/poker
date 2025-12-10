#!/bin/bash

# Script pour lancer le bot Java intelligent

BOT_NAME=$1
GAME_ID=$2

if [ -z "$BOT_NAME" ] || [ -z "$GAME_ID" ]; then
    echo "Usage: ./start-java-bot.sh <nom_du_bot> <game_id>"
    echo ""
    echo "Exemple:"
    echo "  ./start-java-bot.sh SmartBot abc123"
    exit 1
fi

echo "🎯 Vérification du JAR..."

JAR_PATH="java-bot/target/intelligent-bot-1.0.jar"

if [ ! -f "$JAR_PATH" ]; then
    echo "❌ JAR non trouvé. Compilation en cours..."
    cd java-bot
    mvn clean package -q
    STATUS=$?
    cd ..
    
    if [ $STATUS -ne 0 ]; then
        echo "❌ Erreur de compilation"
        exit 1
    fi
    echo "✅ Compilation réussie"
fi

echo "🚀 Lancement du bot $BOT_NAME..."
echo ""

java -jar "$JAR_PATH" "$BOT_NAME" "$GAME_ID"
