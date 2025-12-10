#!/usr/bin/env python3
"""
Bot de poker exemple en Python
Démontre comment se connecter à l'API du serveur de poker
"""

import requests
import time
import random
import sys

API_BASE = "http://localhost:8080/api"

class PokerBot:
    def __init__(self, name, game_id=None):
        self.name = name
        self.game_id = game_id
        self.player_id = None
        
    def create_game(self):
        """Créer une nouvelle partie"""
        data = {
            "name": f"Partie de {self.name}",
            "max_players": 4,
            "starting_chips": 1000,
            "small_blind": 10,
            "big_blind": 20
        }
        
        response = requests.post(f"{API_BASE}/games", json=data)
        if response.status_code == 200:
            result = response.json()
            self.game_id = result["game_id"]
            print(f"✅ Partie créée: {self.game_id}")
            return True
        else:
            print(f"❌ Erreur création: {response.text}")
            return False
    
    def join_game(self):
        """Rejoindre une partie"""
        if not self.game_id:
            print("❌ Aucune partie spécifiée")
            return False
            
        data = {
            "bot_name": self.name
        }
        
        response = requests.post(f"{API_BASE}/games/{self.game_id}/join", json=data)
        if response.status_code == 200:
            result = response.json()
            self.player_id = result["player_id"]
            print(f"✅ Rejoint la partie en position {result['position']}")
            print(f"   Player ID: {self.player_id}")
            return True
        else:
            print(f"❌ Erreur join: {response.text}")
            return False
    
    def get_game_state(self):
        """Obtenir l'état actuel de la partie"""
        if not self.game_id or not self.player_id:
            return None
            
        response = requests.get(
            f"{API_BASE}/games/{self.game_id}/state",
            params={"player_id": self.player_id}
        )
        
        if response.status_code == 200:
            return response.json()
        else:
            return None
    
    def submit_action(self, action_type, amount=None):
        """Soumettre une action"""
        if not self.game_id or not self.player_id:
            return False
            
        action = {"type": action_type}
        if amount is not None:
            action["amount"] = amount
            
        data = {
            "player_id": self.player_id,
            "action": action
        }
        
        response = requests.post(
            f"{API_BASE}/games/{self.game_id}/action",
            json=data
        )
        
        if response.status_code == 200:
            result = response.json()
            if result["success"]:
                print(f"✅ Action {action_type} effectuée")
                return True
            else:
                print(f"❌ Action refusée: {result.get('error', 'Erreur inconnue')}")
                return False
        else:
            print(f"❌ Erreur action: {response.text}")
            return False
    
    def decide_action(self, state):
        """
        Stratégie simple du bot
        Cette fonction peut être personnalisée pour implémenter votre stratégie
        """
        valid_actions = state.get("valid_actions", [])
        
        if not valid_actions:
            return None
        
        # Stratégie aléatoire simple pour la démo
        if "check" in valid_actions:
            # 70% check, 30% raise si possible
            if random.random() < 0.7:
                return ("check", None)
            elif "raise" in valid_actions:
                return ("raise", state["current_bet"] + 20)
            else:
                return ("check", None)
        
        elif "call" in valid_actions:
            # 60% call, 30% raise, 10% fold
            rand = random.random()
            if rand < 0.6:
                return ("call", None)
            elif rand < 0.9 and "raise" in valid_actions:
                return ("raise", state["current_bet"] + 20)
            else:
                return ("fold", None)
        
        elif "fold" in valid_actions:
            return ("fold", None)
        
        return None
    
    def play(self):
        """Boucle principale du bot"""
        print(f"\n🤖 Bot {self.name} en action!")
        
        while True:
            try:
                state = self.get_game_state()
                
                if not state:
                    print("⏸️  En attente de l'état du jeu...")
                    time.sleep(2)
                    continue
                
                # Afficher l'état
                print(f"\n📊 État du jeu:")
                print(f"   Phase: {state['phase']}")
                print(f"   Pot: {state['pot']}")
                print(f"   Vos jetons: {state.get('your_chips', '?')}")
                print(f"   Vos cartes: {state.get('your_cards', [])}")
                print(f"   Cartes communes: {state.get('community_cards', [])}")
                
                # Est-ce notre tour ?
                if state.get("current_player_id") == self.player_id:
                    print(f"\n🎯 C'est notre tour!")
                    print(f"   Actions valides: {state.get('valid_actions', [])}")
                    
                    # Décider de l'action
                    action = self.decide_action(state)
                    
                    if action:
                        action_type, amount = action
                        print(f"   → Décision: {action_type}" + (f" ({amount})" if amount else ""))
                        self.submit_action(action_type, amount)
                    
                    time.sleep(1)
                else:
                    current_player = state.get("current_player_id", "?")
                    print(f"⏳ En attente (tour de {current_player})")
                
                time.sleep(2)
                
            except KeyboardInterrupt:
                print("\n\n👋 Bot arrêté")
                break
            except Exception as e:
                print(f"❌ Erreur: {e}")
                time.sleep(5)

def main():
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python bot_example.py <nom_du_bot> [game_id]")
        print("\nExemples:")
        print("  python bot_example.py MonBot              # Crée une nouvelle partie")
        print("  python bot_example.py MonBot <game_id>    # Rejoint une partie existante")
        sys.exit(1)
    
    bot_name = sys.argv[1]
    game_id = sys.argv[2] if len(sys.argv) > 2 else None
    
    bot = PokerBot(bot_name, game_id)
    
    # Si pas de game_id, créer une partie
    if not game_id:
        if not bot.create_game():
            sys.exit(1)
        print(f"\n💡 Pour rejoindre cette partie avec un autre bot:")
        print(f"   python bot_example.py AutreBot {bot.game_id}")
    
    # Rejoindre la partie
    if not bot.join_game():
        sys.exit(1)
    
    print(f"\n⏳ En attente d'autres joueurs...")
    print(f"   Utilisez la commande suivante pour démarrer la partie:")
    print(f"   curl -X POST http://localhost:8080/api/games/{bot.game_id}/start")
    
    # Jouer
    bot.play()

if __name__ == "__main__":
    main()
