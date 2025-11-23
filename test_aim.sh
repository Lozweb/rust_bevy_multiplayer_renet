#!/bin/bash

# Script de test pour vérifier que le aim_direction fonctionne avec plusieurs clients
# Utilisation: ./test_aim.sh

echo "🚀 Test du aim_direction avec plusieurs clients"
echo "================================================"
echo ""
echo "Instructions:"
echo "1. Le serveur va démarrer"
echo "2. Attendez quelques secondes"
echo "3. Lancez 2 clients manuellement avec: cargo run --bin client"
echo "4. Bougez la souris dans chaque client"
echo "5. Vérifiez que la croix de visée bouge pour tous les joueurs"
echo ""
echo "Démarrage du serveur..."
echo ""

# Démarrer le serveur
cargo run --release --bin server

