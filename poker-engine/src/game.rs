use crate::card::Card;
use crate::deck::Deck;
use crate::hand::Hand;
use serde::{Deserialize, Serialize};

/// Identifiant unique d'un joueur
pub type PlayerId = String;

/// Statut d'un joueur dans la partie
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerStatus {
    Active,      // Joueur actif dans la main
    Folded,      // Joueur qui s'est couché
    AllIn,       // Joueur qui a misé tous ses jetons
    SittingOut,  // Joueur absent
    Eliminated,  // Joueur éliminé (plus de jetons)
}

/// Représente un joueur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub chips: u32,
    pub hole_cards: Vec<Card>,
    pub current_bet: u32,
    pub status: PlayerStatus,
}

impl Player {
    pub fn new(id: PlayerId, name: String, chips: u32) -> Self {
        Player {
            id,
            name,
            chips,
            hole_cards: Vec::new(),
            current_bet: 0,
            status: PlayerStatus::Active,
        }
    }

    /// Mise des jetons
    pub fn bet(&mut self, amount: u32) -> u32 {
        let actual_bet = amount.min(self.chips);
        self.chips -= actual_bet;
        self.current_bet += actual_bet;
        
        if self.chips == 0 {
            self.status = PlayerStatus::AllIn;
        }
        
        actual_bet
    }

    /// Se coucher
    pub fn fold(&mut self) {
        self.status = PlayerStatus::Folded;
    }

    /// Réinitialiser pour une nouvelle main
    pub fn reset_for_new_hand(&mut self) {
        self.hole_cards.clear();
        self.current_bet = 0;
        if self.chips > 0 {
            self.status = PlayerStatus::Active;
        } else {
            self.status = PlayerStatus::Eliminated;
        }
    }
}

/// Phase de jeu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    PreFlop,   // Avant le flop
    Flop,      // Après les 3 premières cartes communes
    Turn,      // Après la 4ème carte commune
    River,     // Après la 5ème carte commune
    Showdown,  // Dévoilement des cartes
}

/// Action possible d'un joueur
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerAction {
    Fold,
    Check,
    Call,
    Raise(u32),
    AllIn,
}

/// État du jeu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub players: Vec<Player>,
    pub community_cards: Vec<Card>,
    pub pot: u32,
    pub current_bet: u32,
    pub phase: GamePhase,
    pub dealer_position: usize,
    pub current_player: usize,
    pub small_blind: u32,
    pub big_blind: u32,
    pub players_acted: Vec<bool>, // Suit qui a agi dans le tour actuel
    
    // Champs d'historique pour l'UI
    pub action_log: Vec<String>,
    pub last_hand_winner: Option<usize>,
    pub last_hand_amount: u32,
    pub last_hand_description: String,
    pub last_hand_winner_name: Option<String>,
    pub last_hand_cards: Option<Vec<Card>>,

    #[serde(skip)]
    deck: Deck,
}

impl GameState {
    /// Crée une nouvelle partie
    pub fn new(player_ids: Vec<(PlayerId, String)>, starting_chips: u32, small_blind: u32, big_blind: u32) -> Self {
        let num_players = player_ids.len();
        let players = player_ids
            .into_iter()
            .map(|(id, name)| Player::new(id, name, starting_chips))
            .collect();

        GameState {
            players,
            community_cards: Vec::new(),
            pot: 0,
            current_bet: 0,
            phase: GamePhase::PreFlop,
            dealer_position: 0,
            current_player: 0,
            small_blind,
            big_blind,
            players_acted: vec![false; num_players],
            deck: Deck::new(),
            
            // Initialisation des champs d'historique
            action_log: Vec::new(),
            last_hand_winner: None,
            last_hand_amount: 0,
            last_hand_description: String::new(),
            last_hand_winner_name: None,
            last_hand_cards: None,
        }
    }

    /// Démarre une nouvelle main
    pub fn start_new_hand(&mut self) {
        // Réinitialiser les joueurs
        for player in &mut self.players {
            player.reset_for_new_hand();
        }

        // Réinitialiser l'état du jeu
        self.community_cards.clear();
        self.pot = 0;
        self.current_bet = 0;
        self.phase = GamePhase::PreFlop;

        // Déplacer le bouton du dealer vers le prochain joueur ACTIF
        self.dealer_position = self.get_next_active_player(self.dealer_position);
        
        // Reset acted status
        self.players_acted = vec![false; self.players.len()];

        // Nouveau paquet mélangé
        self.deck = Deck::new();
        self.deck.shuffle();

        // Poster les blinds
        self.post_blinds();

        // Distribuer les cartes
        self.deal_hole_cards();

        // Le premier joueur après la big blind commence (UTG)
        // Dealer -> SB -> BB -> UTG
        let sb_pos = self.get_next_active_player(self.dealer_position);
        let bb_pos = self.get_next_active_player(sb_pos);
        self.current_player = self.get_next_active_player(bb_pos);

        // Si tout le monde est déjà All-In (ou éliminé/seul), avancer
        let active_count = self.players.iter().filter(|p| p.status == PlayerStatus::Active).count();
        if active_count == 0 {
            println!("🚀 Tous les joueurs All-In/Eliminés dès le départ -> Auto Advance");
            self.advance_phase();
        }
    }

    /// Poster les blinds
    fn post_blinds(&mut self) {
        // Trouver SB et BB parmi les joueurs ACTIFS uniquement
        let sb_pos = self.get_next_active_player(self.dealer_position);
        let bb_pos = self.get_next_active_player(sb_pos);

        // Si HEADS-UP (2 joueurs actifs), le Dealer est SB, l'autre est BB
        // La logique standard next(dealer) -> SB marche si on considère que le dealer joue !
        // En Heads-up: Dealer = SB, Autre = BB.
        // Ma logique actuelle: Dealer -> next -> SB.
        // Si Dealer est actif, next(dealer) = l'autre joueur.
        // Donc en HU normal: Dealer(SB) -> Opponent(BB).
        // Mais ma fonction: next(dealer) saute le dealer s'il est actif ? 
        // get_next_active_player fait (idx+1)%len. 
        // Si Dealer est P1. next(P1) = P2.
        // Donc SB = P2. Ce qui est l'inverse du HU standard (où Dealer = SB).
        // TODO: Ajustement pour Heads-Up plus tard si besoin, restons sur la logique standard pour l'instant.

        let sb_amount = self.players[sb_pos].bet(self.small_blind);
        self.pot += sb_amount;

        let bb_amount = self.players[bb_pos].bet(self.big_blind);
        self.pot += bb_amount;
        self.current_bet = self.big_blind;
    }

    /// Distribuer les cartes privées
    fn deal_hole_cards(&mut self) {
        for _ in 0..2 {
            for player in &mut self.players {
                // Ne distribuer qu'aux joueurs actifs
                if player.status == PlayerStatus::Active {
                    if let Some(card) = self.deck.deal() {
                        player.hole_cards.push(card);
                    }
                }
            }
        }
    }

    /// Exécuter une action de joueur
    pub fn execute_action(&mut self, player_id: &PlayerId, action: PlayerAction) -> Result<(), String> {
        let player_idx = self.players
            .iter()
            .position(|p| &p.id == player_id)
            .ok_or("Joueur non trouvé")?;

        if player_idx != self.current_player {
            return Err("Ce n'est pas le tour de ce joueur".to_string());
        }

        let player = &mut self.players[player_idx];
        
        // CORRECTION: Vérifier que le joueur peut agir
        if player.status != PlayerStatus::Active {
            return Err(format!("Le joueur ne peut pas agir (statut: {:?})", player.status));
        }

        let player_name = player.name.clone();

        match action {
            PlayerAction::Fold => {
                player.fold();
                self.players_acted[player_idx] = true;
                self.action_log.push(format!("{} folds", player_name));
            }
            PlayerAction::Check => {
                if player.current_bet < self.current_bet {
                    return Err("Impossible de checker, il faut suivre ou se coucher".to_string());
                }
                self.players_acted[player_idx] = true;
                self.action_log.push(format!("{} checks", player_name));
            }
            PlayerAction::Call => {
                let call_amount = self.current_bet - player.current_bet;
                let actual_bet = player.bet(call_amount);
                self.pot += actual_bet;
                self.players_acted[player_idx] = true;
                self.action_log.push(format!("{} calls {}", player_name, actual_bet));
            }
            PlayerAction::Raise(amount) => {
                // CORRECTION: Vérifier le montant minimum de raise
                let min_raise = self.big_blind.max(self.current_bet);
                if amount < min_raise {
                    return Err(format!(
                        "Raise trop petit. Minimum: {} (big blind: {}, current bet: {})",
                        min_raise, self.big_blind, self.current_bet
                    ));
                }
                
                let total_bet = self.current_bet + amount;
                let to_call = self.current_bet - player.current_bet;
                let actual_bet = player.bet(to_call + amount);
                self.pot += actual_bet;
                self.current_bet = total_bet;
                
                // Reset acted for everyone else because of the raise
                for i in 0..self.players_acted.len() {
                    self.players_acted[i] = false;
                }
                self.players_acted[player_idx] = true;
                self.action_log.push(format!("{} raises to {}", player_name, total_bet));
            }
            PlayerAction::AllIn => {
                let all_in_amount = player.chips;
                let actual_bet = player.bet(all_in_amount);
                self.pot += actual_bet;
                
                if player.current_bet > self.current_bet {
                    self.current_bet = player.current_bet;
                    // Reset acted if raise
                    for i in 0..self.players_acted.len() {
                        self.players_acted[i] = false;
                    }
                }
                self.players_acted[player_idx] = true;
                self.action_log.push(format!("{} goes all-in with {}", player_name, all_in_amount));
            }
        }

        // CORRECTION CRITIQUE: Vérifier si le tour est terminé AVANT de chercher le joueur suivant
        // Cela évite de bloquer quand tous sont foldés/all-in
        if self.is_betting_round_complete() {
            // Si un seul joueur reste et que personne n'est All-In, il gagne immédiatement
            if self.check_sole_survivor() {
                println!("🏆 Un seul survivant - Fin de main anticipée");
                self.end_hand_early();
            } else {
                self.advance_phase();
            }
        } else {
            self.advance_to_next_player();
        }
        
        Ok(())
    }

    /// Passer au joueur suivant
    fn advance_to_next_player(&mut self) {
        let starting_player = self.current_player;
        let mut iterations = 0;
        let max_iterations = self.players.len() + 1;
        
        loop {
            self.current_player = (self.current_player + 1) % self.players.len();
            iterations += 1;
            
            // Sécurité: éviter les boucles infinies
            if iterations >= max_iterations {
                if self.is_betting_round_complete() {
                    self.advance_phase();
                    return;
                }
                // Si vraiment bloqué, forcer l'avancement
                println!("⚠️  WARNING: Aucun joueur actif trouvé, avancement forcé");
                self.advance_phase();
                return;
            }
            
            // Si on a fait un tour complet, vérifier si le tour de mise est terminé
            if self.current_player == starting_player {
                if self.is_betting_round_complete() {
                    self.advance_phase();
                    return;
                }
            }

            // CORRECTION: Si le joueur est actif, c'est son tour
            if self.players[self.current_player].status == PlayerStatus::Active {
                break;
            }
        }
    }

    /// Vérifie si le tour d'enchères est terminé
    fn is_betting_round_complete(&self) -> bool {
        let active_players: Vec<usize> = self.players
            .iter()
            .enumerate()
            .filter(|(_, p)| p.status == PlayerStatus::Active)
            .map(|(i, _)| i)
            .collect();

        // S'il ne reste qu'un joueur (ou 0), le tour est fini
        if active_players.len() <= 1 {
            return true;
        }

        // Vérifier si tous les joueurs actifs ont la même mise
        let stakes_equal = active_players
            .iter()
            .all(|&i| self.players[i].current_bet == self.current_bet || self.players[i].chips == 0); // All-in exception

        // Vérifier si tous les joueurs actifs ont agi
        let all_acted = active_players
            .iter()
            .all(|&i| self.players_acted[i] || self.players[i].chips == 0); // All-in exception

        stakes_equal && all_acted
    }

    /// Avancer à la phase suivante
    fn advance_phase(&mut self) {
        // Réinitialiser les mises des joueurs
        for player in &mut self.players {
            player.current_bet = 0;
        }
        self.current_bet = 0;

        match self.phase {
            GamePhase::PreFlop => {
                self.phase = GamePhase::Flop;
                self.deal_flop();
                self.current_player = self.get_next_active_player(self.dealer_position);
            }
            GamePhase::Flop => {
                self.phase = GamePhase::Turn;
                self.deal_turn();
                self.current_player = self.get_next_active_player(self.dealer_position);
            }
            GamePhase::Turn => {
                self.phase = GamePhase::River;
                self.deal_river();
                self.current_player = self.get_next_active_player(self.dealer_position);
            }
            GamePhase::River => {
                self.phase = GamePhase::Showdown;
                self.showdown();
            }
            GamePhase::Showdown => {
                // La main est terminée
            }
        }
        
        // AUTO-ADVANCE: Si tous les joueurs restants sont all-in ou foldés,
        // continuer automatiquement jusqu'au showdown
        if self.phase != GamePhase::Showdown {
            let active_players: Vec<_> = self.players
                .iter()
                .filter(|p| p.status == PlayerStatus::Active)
                .collect();
            
            // S'il n'y a plus de joueurs actifs (tous all-in ou foldés), avancer automatiquement
            if active_players.is_empty() {
                println!("🚀 AUTO-ADVANCE: Tous les joueurs sont all-in, avancement automatique vers le showdown");
                self.advance_phase();
            }
        }
    }

    /// Distribuer le flop (3 cartes communes)
    fn deal_flop(&mut self) {
        self.deck.deal(); // Brûler une carte
        for _ in 0..3 {
            if let Some(card) = self.deck.deal() {
                self.community_cards.push(card);
            }
        }
    }

    /// Distribuer le turn (4ème carte commune)
    fn deal_turn(&mut self) {
        self.deck.deal(); // Brûler une carte
        if let Some(card) = self.deck.deal() {
            self.community_cards.push(card);
        }
    }

    /// Distribuer la river (5ème carte commune)
    fn deal_river(&mut self) {
        self.deck.deal(); // Brûler une carte
        if let Some(card) = self.deck.deal() {
            self.community_cards.push(card);
        }
    }

    /// Abattage et détermination du gagnant
    fn showdown(&mut self) {
        let mut player_hands: Vec<(usize, Hand)> = Vec::new();

        for (idx, player) in self.players.iter().enumerate() {
            if player.status != PlayerStatus::Folded {
                let mut all_cards = player.hole_cards.clone();
                all_cards.extend(&self.community_cards);
                
                let hand = Hand::evaluate(&all_cards);
                player_hands.push((idx, hand));
            }
        }

        // Trouver le meilleur main
        if let Some((winner_idx, _)) = player_hands.iter().max_by(|(_, h1), (_, h2)| h1.cmp(h2)) {
            self.players[*winner_idx].chips += self.pot;
            self.pot = 0;
        }
        
        // CORRECTION: Démarrer automatiquement la prochaine main
        println!("🎴 SHOWDOWN terminé - Démarrage nouvelle main");
        self.start_new_hand();
    }

    /// Vérifie s'il ne reste qu'un seul joueur survivant (tous les autres foldés)
    /// Retourne false si d'autres joueurs sont All-In (car il faut aller au showdown)
    fn check_sole_survivor(&self) -> bool {
        let active_count = self.players.iter().filter(|p| p.status == PlayerStatus::Active).count();
        let all_in_count = self.players.iter().filter(|p| p.status == PlayerStatus::AllIn).count();
        
        // S'il n'y a qu'un actif et personne à tapis, c'est un survivant unique
        active_count == 1 && all_in_count == 0
    }

    /// Obtenir l'index du prochain joueur actif (en sautant les éliminés)
    fn get_next_active_player(&self, start_idx: usize) -> usize {
        let mut idx = start_idx;
        let num_players = self.players.len();
        // On boucle au maximum une fois le tour de table
        for _ in 0..num_players {
            idx = (idx + 1) % num_players;
            if self.players[idx].status == PlayerStatus::Active {
                return idx;
            }
        }
        idx // Retourne start_idx si personne d'autre n'est trouvé (bug ?)
    }

    /// Terminer la main de manière anticipée (tout le monde s'est couché)
    fn end_hand_early(&mut self) {
        // Trouver le gagnant (le seul actif)
        if let Some(winner_idx) = self.players.iter().position(|p| p.status == PlayerStatus::Active) {
            let winner_name = self.players[winner_idx].name.clone();
            println!("🎉 Victoire par forfait de {}", winner_name);
            
            // Donner le pot
            self.players[winner_idx].chips += self.pot;
            
            // Marquer le gagnant dans l'historique (pour le log UI)
            self.action_log.push(format!("{} wins {} chips (opponent folded)", winner_name, self.pot));
            self.last_hand_winner = Some(winner_idx);
            self.last_hand_amount = self.pot;
            self.last_hand_description = "Adversaire couché".to_string();
            self.last_hand_winner_name = Some(winner_name.clone());
            self.last_hand_cards = Some(vec![]); // Pas de cartes à montrer
            
            self.pot = 0;
            
            // Démarrer info nouvelle main
            self.start_new_hand();
        } else {
            // Cas impossible théoriquement
            println!("❌ ERREUR: Aucun gagnant trouvé pour fin anticipée");
            self.start_new_hand();
        }
    }

    /// Obtenir les actions valides pour le joueur actuel
    pub fn get_valid_actions(&self) -> Vec<PlayerAction> {
        // CORRECTION: Au showdown, aucune action n'est possible
        if self.phase == GamePhase::Showdown {
            return vec![];
        }
        
        let player = &self.players[self.current_player];
        
        // CORRECTION: Les joueurs non-actifs ne peuvent pas agir
        if player.status != PlayerStatus::Active {
            return vec![];
        }
        
        let mut actions = vec![PlayerAction::Fold];

        let to_call = self.current_bet - player.current_bet;

        if to_call == 0 {
            actions.push(PlayerAction::Check);
            // CORRECTION: Raise minimum = big blind
            if player.chips > self.big_blind {
                actions.push(PlayerAction::Raise(self.big_blind));
            }
        } else if player.chips >= to_call {
            actions.push(PlayerAction::Call);
            // CORRECTION: Raise minimum = soit la big blind, soit le montant de la dernière raise
            let min_raise = self.big_blind.max(self.current_bet);
            if player.chips > to_call + min_raise {
                actions.push(PlayerAction::Raise(min_raise));
            }
        }

        // All-in toujours disponible si on a des jetons
        if player.chips > 0 {
            actions.push(PlayerAction::AllIn);
        }

        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let players = vec![
            ("p1".to_string(), "Alice".to_string()),
            ("p2".to_string(), "Bob".to_string()),
        ];
        let game = GameState::new(players, 1000, 10, 20);
        
        assert_eq!(game.players.len(), 2);
        assert_eq!(game.small_blind, 10);
        assert_eq!(game.big_blind, 20);
    }

    #[test]
    fn test_start_new_hand() {
        let players = vec![
            ("p1".to_string(), "Alice".to_string()),
            ("p2".to_string(), "Bob".to_string()),
        ];
        let mut game = GameState::new(players, 1000, 10, 20);
        game.start_new_hand();

        // Vérifier que les cartes ont été distribuées
        assert_eq!(game.players[0].hole_cards.len(), 2);
        assert_eq!(game.players[1].hole_cards.len(), 2);

        // Vérifier que les blinds ont été postées
        assert!(game.pot > 0);
    }

    #[test]
    fn test_player_bet() {
        let mut player = Player::new("p1".to_string(), "Alice".to_string(), 1000);
        let bet_amount = player.bet(100);
        
        assert_eq!(bet_amount, 100);
        assert_eq!(player.chips, 900);
        assert_eq!(player.current_bet, 100);
    }

    #[test]
    fn test_player_fold() {
        let mut player = Player::new("p1".to_string(), "Alice".to_string(), 1000);
        player.fold();
        
        assert_eq!(player.status, PlayerStatus::Folded);
    }


    #[test]
    fn test_fold_bug_repro() {
        let players = vec![
            ("p1".to_string(), "Alice".to_string()),
            ("p2".to_string(), "Bob".to_string()),
            ("p3".to_string(), "Charlie".to_string()),
        ];
        // Blinds: 10/20. Chips: 1000.
        let mut game = GameState::new(players, 1000, 10, 20);
        game.start_new_hand();

        // Hand 1 starts.
        // Dealer moves to next active after 0 -> 1 (Bob).
        // Dealer = 1 (Bob).
        // SB = 2 (Charlie).
        // BB = 0 (Alice).
        // UTG = 1 (Bob).

        assert_eq!(game.dealer_position, 1, "Dealer should be Bob (1)");
        assert_eq!(game.current_player, 1, "UTG (Bob) starts");

        // 1. Bob (UTG) Calls 20
        game.execute_action(&"p2".to_string(), PlayerAction::Call).unwrap();

        // 2. Charlie (SB) Folds!
        // This is the key: The player immediately after Dealer folds.
        game.execute_action(&"p3".to_string(), PlayerAction::Fold).unwrap();
        assert_eq!(game.players[2].status, PlayerStatus::Folded);

        // 3. Alice (BB) Checks
        game.execute_action(&"p1".to_string(), PlayerAction::Check).unwrap();

        // Preflop complete. Advance to Flop.
        assert_eq!(game.phase, GamePhase::Flop);

        // BUG CHECK:
        // Dealer is Bob (1).
        // Naive logic: (1 + 1) % 3 = 2 (Charlie).
        // But Charlie is FOLDED.
        // Correct logic: Should skip Charlie and go to Alice (0).
        
        assert_ne!(game.current_player, 2, "Game blocked: Turn assigned to folded player");
        assert_eq!(game.current_player, 0, "Turn should skip folded player (Charlie) and go to Alice");
    }
}
