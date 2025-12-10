#!/bin/bash

# Script pour lancer le serveur de poker

echo "🚀 Démarrage du serveur de poker..."

# Se placer dans le répertoire du script
cd "$(dirname "$0")"

# Utiliser le chemin complet de cargo
~/.cargo/bin/cargo run --bin poker-server
