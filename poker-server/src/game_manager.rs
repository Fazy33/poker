use crate::models::*;
use crate::auth::{create_token};  // Importer la fonction de création de token
use poker_engine::GameState;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Salle de jeu contenant l'état et les métadonnées
#[derive(Clone)]
pub struct GameRoom {
    pub id: GameId,
    pub name: String,
    pub max_players: usize,
    pub state: GameState,
    pub player_names: HashMap<PlayerId, String>,
    pub player_types: HashMap<PlayerId, PlayerType>,
    pub started: bool,
    pub game_finished: bool,
    pub winner_id: Option<PlayerId>,
    pub action_log: Vec<String>,
    pub last_hand_winner: Option<PlayerId>,
    pub last_hand_amount: Option<u32>,
    pub last_hand_description: Option<String>,
    pub last_hand_cards: Option<Vec<String>>,
    pub last_action_time: std::time::SystemTime,
    pub player_timeout_strikes: HashMap<PlayerId, u32>,
}

impl GameRoom {
    pub fn new(id: GameId, name: String, max_players: usize, starting_chips: u32, small_blind: u32, big_blind: u32) -> Self {
        GameRoom {
            id,
            name,
            max_players,
            state: GameState::new(vec![], starting_chips, small_blind, big_blind),
            player_names: HashMap::new(),
            player_types: HashMap::new(),
            started: false,
            game_finished: false,
            winner_id: None,
            action_log: Vec::new(),
            last_hand_winner: None,
            last_hand_amount: None,
            last_hand_description: None,
            last_hand_cards: None,
            last_action_time: std::time::SystemTime::now(),
            player_timeout_strikes: HashMap::new(),
        }
    }

    /// Ajouter un joueur à la partie
    pub fn add_player(&mut self, player_id: PlayerId, name: String, player_type: PlayerType) -> Result<usize, String> {
        if self.started {
            return Err("La partie a déjà commencé".to_string());
        }

        if self.state.players.len() >= self.max_players {
            return Err("La partie est pleine".to_string());
        }

        if self.player_names.contains_key(&player_id) {
            return Err("Ce joueur est déjà dans la partie".to_string());
        }

        let position = self.state.players.len();
        self.player_names.insert(player_id.clone(), name.clone());
        self.player_types.insert(player_id.clone(), player_type);
        
        // Ajouter le joueur au GameState
        let starting_chips = self.state.players.first()
            .map(|p| p.chips)
            .unwrap_or(1000);
        
        self.state.players.push(poker_engine::Player::new(
            player_id.clone(),
            name.clone(),
            starting_chips,
        ));

        println!("✅ {} a rejoint la partie {} ({}/{})", 
            name, 
            self.name,
            self.state.players.len(), 
            self.max_players
        );

        Ok(position)
    }

    /// Démarrer la partie
    pub fn start_game(&mut self) -> Result<(), String> {
        if self.started {
            return Err("La partie a déjà commencé".to_string());
        }

        if self.state.players.len() < 2 {
            return Err("Il faut au moins 2 joueurs pour commencer".to_string());
        }

        self.started = true;
        self.last_action_time = std::time::SystemTime::now(); // Reset du timer
        self.state.start_new_hand();
        
        // Log de démarrage
        let player_list: Vec<_> = self.state.players.iter()
            .map(|p| self.player_names.get(&p.id).cloned().unwrap_or_else(|| p.name.clone()))
            .collect();
        
        println!("\n╔══════════════════════════════════════╗");
        println!("║   🎮 PARTIE DÉMARRÉE                 ║");
        println!("╚══════════════════════════════════════╝");
        println!("📍 ID: {}", self.id);
        println!("👥 Joueurs ({}):", self.state.players.len());
        for (i, name) in player_list.iter().enumerate() {
            println!("   {}. {}", i + 1, name);
        }
        println!(""); 
        
        Ok(())
    }

    /// Obtenir l'état du jeu pour un joueur spécifique
    pub fn get_state_for_player(&self, player_id: &PlayerId) -> GameStateResponse {
        let player = self.state.players.iter().find(|p| &p.id == player_id);
        
        let your_cards = player.map(|p| {
            p.hole_cards.iter().map(card_to_string).collect()
        });

        let your_chips = player.map(|p| p.chips);

        let current_player_id = if self.started {
            Some(self.state.players[self.state.current_player].id.clone())
        } else {
            None
        };

        let valid_actions = if self.started && current_player_id.as_ref() == Some(player_id) {
            self.state.get_valid_actions()
                .iter()
                .map(|a| match a {
                    poker_engine::PlayerAction::Fold => "fold".to_string(),
                    poker_engine::PlayerAction::Check => "check".to_string(),
                    poker_engine::PlayerAction::Call => "call".to_string(),
                    poker_engine::PlayerAction::Raise(_) => "raise".to_string(),
                    poker_engine::PlayerAction::AllIn => "allin".to_string(),
                })
                .collect()
        } else {
            vec![]
        };

        // SÉCURITÉ: Mode spectateur supprimé - aucune carte d'adversaire n'est jamais envoyée

        GameStateResponse {
            game_id: self.id,
            phase: phase_to_string(&self.state.phase),
            pot: if self.game_finished {
                // Si la partie est finie, le "pot" affiché est le total des gains (tous les jetons du vainqueur)
                self.winner_id.as_ref()
                    .and_then(|wid| self.state.players.iter().find(|p| &p.id == wid))
                    .map(|p| p.chips)
                    .unwrap_or(self.state.pot)
            } else {
                self.state.pot
            },
            current_bet: self.state.current_bet,
            community_cards: self.state.community_cards.iter().map(card_to_string).collect(),
            players: self.state.players.iter().map(|p| {
                // SÉCURITÉ: Montrer les cartes seulement si c'est le joueur lui-même
                // OU si le demandeur n'est pas un joueur (mode spectateur/admin)
                let is_requester_player = self.state.players.iter().any(|pl| pl.id == *player_id);
                let show_cards = p.id == *player_id || !is_requester_player;
                
                PlayerInfo {
                    id: p.id.clone(),
                    name: self.player_names.get(&p.id).cloned().unwrap_or_else(|| p.name.clone()),
                    chips: p.chips,
                    current_bet: p.current_bet,
                    status: format!("{:?}", p.status),
                    player_type: *self.player_types.get(&p.id).unwrap_or(&PlayerType::Bot),
                    cards: if show_cards {
                        Some(p.hole_cards.iter().map(card_to_string).collect())
                    } else {
                        None
                    },
                }
            }).collect(),
            current_player_id,
            your_player_id: Some(player_id.clone()),
            your_chips,
            your_cards,
            valid_actions,
            game_finished: self.game_finished,
            winner_id: self.winner_id.clone(),
            winner_name: self.winner_id.as_ref()
                .and_then(|id| self.player_names.get(id).cloned()),
            // Toujours envoyer le log (limité aux 50 dernières actions pour ne pas surcharger)
            action_log: Some(
                self.action_log.iter()
                    .rev()
                    .take(50)
                    .rev()
                    .cloned()
                    .collect()
            ),
            last_hand_winner: self.last_hand_winner.clone(),
            last_hand_winner_name: self.last_hand_winner.as_ref()
                .and_then(|id| self.player_names.get(id).cloned()),
            last_hand_amount: self.last_hand_amount,
            last_hand_description: self.last_hand_description.clone(),
            last_hand_cards: self.last_hand_cards.clone(),
        }
    }

    /// Obtenir un résumé de la partie
    pub fn get_summary(&self) -> GameSummary {
        GameSummary {
            game_id: self.id,
            name: self.name.clone(),
            player_count: self.state.players.len(),
            max_players: self.max_players,
            phase: phase_to_string(&self.state.phase),
            pot: self.state.pot,
        }
    }

    /// Vérifier les timeouts
    pub fn check_timeouts(&mut self) -> bool {
        if !self.started || self.game_finished {
            return false;
        }

        // Temps max par tour : 30 secondes
        let timeout_duration = std::time::Duration::from_secs(30);
        
        if let Ok(elapsed) = self.last_action_time.elapsed() {
            if elapsed > timeout_duration {
                let current_player_idx = self.state.current_player;
                
                // Vérifier si l'index est valide
                if current_player_idx >= self.state.players.len() {
                    return false;
                }

                let player_id = self.state.players[current_player_idx].id.clone();
                let player_name = self.player_names.get(&player_id)
                    .cloned()
                    .unwrap_or_else(|| "Joueur Inconnu".to_string());

                println!("⏰ TEMPS ÉCOULÉ pour {} !", player_name);
                
                // Incrémenter les strikes
                let strikes = self.player_timeout_strikes.entry(player_id.clone()).or_insert(0);
                *strikes += 1;
                
                let is_ejected = *strikes >= 3;
                
                if is_ejected {
                    println!("🚫 {} a été éjecté de la table (3 timeouts consécutifs)", player_name);
                    self.action_log.push(format!("🚫 {} ejected (too many timeouts)", player_name));
                    
                    // Marquer comme SittingOut ou Eliminated dans le moteur pour qu'il soit ignoré
                    self.state.players[current_player_idx].status = poker_engine::PlayerStatus::Eliminated;
                } else {
                    println!("⚠️ {} fold automatiquement (Strike {}/3)", player_name, strikes);
                    self.action_log.push(format!("⏰ {} timeout (fold) [{}/3]", player_name, strikes));
                }

                // Forcer le Fold (ou juste passer le tour si éjecté)
                // On utilise execute_action avec Fold. Si déjà éliminé, le moteur pourrait rejeter,
                // mais on a besoin de faire avancer le jeu.
                // Si on l'a marqué éliminé manuellement, il faut quand même faire avancer le `current_player`.
                
                // SÉCURITÉ: On reset le timestamp pour ne pas boucler infiniment si ça plante
                self.last_action_time = std::time::SystemTime::now();

                // Tentative de Fold propre via le moteur
                // Si le joueur est ejected, le Fold est technique pour passer au suivant.
                match self.state.execute_action(&player_id, poker_engine::PlayerAction::Fold) {
                    Ok(_) => {
                        println!("✅ Auto-fold exécuté avec succès");
                        
                        // Si éjecté, s'assurer qu'il reste Eliminated (le moteur l'a peut-être mis Folded)
                        if is_ejected {
                             self.state.players[current_player_idx].status = poker_engine::PlayerStatus::Eliminated;
                        }

                        // Vérifier victoire par forfait
                        let active_count = self.state.players.iter()
                            .filter(|p| p.status != poker_engine::PlayerStatus::Eliminated && p.status != poker_engine::PlayerStatus::SittingOut)
                            .count();
                            
                        if active_count <= 1 {
                             let winner = self.state.players.iter()
                                .find(|p| p.status != poker_engine::PlayerStatus::Eliminated && p.status != poker_engine::PlayerStatus::SittingOut);
                                
                             if let Some(w) = winner {
                                 self.game_finished = true;
                                 self.winner_id = Some(w.id.clone());
                                 println!("🏆 Victoire par forfait de {}", w.name);
                             }
                        }
                    },
                    Err(e) => {
                         println!("❌ Erreur lors de l'auto-fold: {}", e);
                         // Si le moteur refuse (ex: pas le bon tour), on force l'avancement manuellement ??
                         // C'est risqué. Mieux vaut espérer que le fix du moteur précédent (fold bug) gère ça.
                    }
                }
                
                return true; // Une action a été prise
            }
        }
        
        false
    }
}

/// Gestionnaire de toutes les parties
pub struct GameManager {
    games: Arc<Mutex<HashMap<GameId, GameRoom>>>,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            games: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Créer une nouvelle partie
    pub fn create_game(&self, req: CreateGameRequest) -> Result<CreateGameResponse, String> {
        let game_id = Uuid::new_v4();
        let game_room = GameRoom::new(
            game_id,
            req.name.clone(),
            req.max_players,
            req.starting_chips,
            req.small_blind,
            req.big_blind,
        );

        let mut games = self.games.lock().unwrap();
        games.insert(game_id, game_room);

        Ok(CreateGameResponse {
            game_id,
            name: req.name,
        })
    }

    /// Rejoindre une partie
    pub fn join_game(&self, game_id: GameId, req: JoinGameRequest) -> Result<JoinGameResponse, String> {
        let mut games = self.games.lock().unwrap();
        let game = games.get_mut(&game_id)
            .ok_or("Partie non trouvée")?;

        // Utiliser un nom différent selon le type de joueur
        let display_name = match req.player_type {
            PlayerType::Human => req.bot_name.clone(),
            PlayerType::Bot => req.bot_name.clone(),
        };

        let player_id = format!("{}_{}", req.bot_name, Uuid::new_v4());
        let position = game.add_player(player_id.clone(), display_name, req.player_type)?;

        // Générer un token JWT pour ce joueur
        let auth_token = create_token(&player_id, &game_id.to_string())
            .map_err(|e| format!("Erreur de génération de token: {}", e))?;

        Ok(JoinGameResponse {
            player_id,
            game_id,
            position,
            auth_token,
        })
    }

    /// Soumettre une action
    pub fn submit_action(&self, game_id: GameId, req: SubmitActionRequest) -> Result<SubmitActionResponse, String> {
        // SÉCURITÉ: Vérifier le token JWT et extraire le player_id
        let claims = crate::auth::verify_token(&req.auth_token)?;
        
        if claims.game_id != game_id.to_string() {
            return Err("Token invalide pour cette partie".to_string());
        }
        
        let player_id = claims.player_id;
        
        let mut games = self.games.lock().unwrap();
        let game = games.get_mut(&game_id)
            .ok_or("Partie non trouvée")?;

        if !game.started {
            return Err("La partie n'a pas encore commencé".to_string());
        }

        if game.game_finished {
            return Err("La partie est terminée".to_string());
        }

        let player_name = game.player_names.get(&player_id)
            .cloned()
            .unwrap_or_else(|| player_id.clone());

        // LOG: Action reçue
        println!("📥 Action reçue de {} -> {:?}", player_name, req.action);
        
        let engine_action = req.action.into();
        
        // Logger l'action
        let action_desc = match &engine_action {
            poker_engine::PlayerAction::Fold => "FOLD".to_string(),
            poker_engine::PlayerAction::Check => "CHECK".to_string(),
            poker_engine::PlayerAction::Call => format!("CALL ({})", game.state.current_bet),
            poker_engine::PlayerAction::Raise(amount) => format!("RAISE ({})", amount),
            poker_engine::PlayerAction::AllIn => "ALL-IN".to_string(),
        };
        
        let log_entry = format!(
            "[{}] {} -> {}",
            phase_to_string(&game.state.phase),
            player_name,
            action_desc
        );
        game.action_log.push(log_entry.clone());
        
        // LOG: État avant l'action
        println!("🎲 État: Phase={}, Pot={}, CurrentBet={}, CurrentPlayer={}", 
            phase_to_string(&game.state.phase),
            game.state.pot,
            game.state.current_bet,
            game.state.current_player
        );

        // Capturer le pot AVANT le showdown (car il sera distribué après)
        let pot_before_action = game.state.pot;
        let was_river_phase = game.state.phase == poker_engine::GamePhase::River;

        match game.state.execute_action(&player_id, engine_action) {
            Ok(_) => {
                // SÉCURITÉ: Reset du timer et des strikes car le joueur a joué
                game.last_action_time = std::time::SystemTime::now();
                game.player_timeout_strikes.remove(&player_id);

                // LOG: Action exécutée avec succès
                println!("✅ {} | Pot maintenant: {}", log_entry, game.state.pot);
                
                // LOG: Nouveau joueur actif
                let next_player = &game.state.players[game.state.current_player];
                let next_player_name = game.player_names.get(&next_player.id)
                    .cloned()
                    .unwrap_or_else(|| next_player.id.clone());
                println!("👉 Tour suivant: {} (Phase: {})", 
                    next_player_name,
                    phase_to_string(&game.state.phase)
                );
                
                // Vérifier si un seul joueur a encore des jetons
                let players_with_chips: Vec<_> = game.state.players.iter()
                    .filter(|p| p.chips > 0)
                    .collect();
                
                if players_with_chips.len() == 1 {
                    game.game_finished = true;
                    game.winner_id = Some(players_with_chips[0].id.clone());
                    
                    // Afficher le log complet de la partie
                    println!("\n════════════════════════════════════════");
                    println!("🏆 PARTIE TERMINÉE - LOG COMPLET");
                    println!("════════════════════════════════════════");
                    println!("Gagnant: {}", game.player_names.get(&players_with_chips[0].id)
                        .unwrap_or(&players_with_chips[0].id));
                    println!("\n📋 Historique des actions:\n");
                    for (i, entry) in game.action_log.iter().enumerate() {
                        println!("  {}. {}", i + 1, entry);
                    }
                    println!("\n════════════════════════════════════════\n");
                    
                    return Ok(SubmitActionResponse {
                        success: true,
                        error: None,
                    });
                }
                
                // Si on vient de passer en showdown depuis River, capturer le gagnant
                if was_river_phase && game.state.phase == poker_engine::GamePhase::Showdown {
                    // Trouver le gagnant en comparant les jetons avant/après
                    let active_players: Vec<_> = game.state.players.iter()
                        .filter(|p| p.status != poker_engine::PlayerStatus::Folded)
                        .collect();
                    
                    if !active_players.is_empty() {
                        // Trouver qui a gagné le pot (celui qui a le plus de jetons maintenant)
                        // Note: pot_before_action contient le montant qui sera distribué
                        if let Some(winner) = active_players.iter().max_by_key(|p| p.chips) {
                            game.last_hand_winner = Some(winner.id.clone());
                            game.last_hand_amount = Some(pot_before_action);
                            game.last_hand_description = Some(format!("Main gagnante"));
                            // Capturer les cartes du gagnant
                            game.last_hand_cards = Some(
                                winner.hole_cards.iter().map(card_to_string).collect()
                            );
                            
                            println!("🎊 SHOWDOWN - {} gagne {} jetons avec {:?}", 
                                game.player_names.get(&winner.id).unwrap_or(&winner.id),
                                pot_before_action,
                                winner.hole_cards
                            );
                        }
                    }
                    
                    println!("🎊 SHOWDOWN terminé - Nouvelle main commence dans 5 secondes");
                    game.action_log.push(format!("[SHOWDOWN] Nouveau tour commence"));
                    
                    // Démarrer la nouvelle main
                    game.state.start_new_hand();
                    
                    // Réinitialiser les infos de la dernière main après 1 cycle
                    // (elles seront affichées pendant 5s côté client puis effacées)
                    
                    // LOG: État après nouvelle main
                    println!("🆕 Nouvelle main: Phase={}, Pot={}, Dealer={}", 
                        phase_to_string(&game.state.phase),
                        game.state.pot,
                        game.state.dealer_position
                    );
                }
                
                Ok(SubmitActionResponse {
                    success: true,
                    error: None,
                })
            },
            Err(e) => {
                // LOG: Erreur
                println!("❌ Action refusée de {}: {}", player_name, e);
                Ok(SubmitActionResponse {
                    success: false,
                    error: Some(e),
                })
            },
        }
    }

    /// Démarrer une partie
    pub fn start_game(&self, game_id: GameId) -> Result<(), String> {
        println!("⚡ Demande de démarrage de partie reçue pour {}", game_id);
        
        let mut games = self.games.lock().unwrap();
        let game = games.get_mut(&game_id)
            .ok_or("Partie non trouvée")?;
        game.start_game()
    }

    /// Obtenir l'état d'une partie pour un joueur
    pub fn get_game_state(&self, game_id: GameId, player_id: &PlayerId) -> Result<GameStateResponse, String> {
        let games = self.games.lock().unwrap();
        let game = games.get(&game_id)
            .ok_or("Partie non trouvée")?;
        Ok(game.get_state_for_player(player_id))
    }

    /// Lister toutes les parties
    pub fn list_games(&self) -> GameListResponse {
        let games = self.games.lock().unwrap();
        let summaries = games.values()
            .map(|g| g.get_summary())
            .collect();
        
        GameListResponse {
            games: summaries,
        }
    }

    /// Obtenir une copie d'une partie (pour WebSocket)
    #[allow(dead_code)] // Pour usage futur (WebSocket)
    pub fn get_game(&self, game_id: GameId) -> Option<GameRoom> {
        let games = self.games.lock().unwrap();
        games.get(&game_id).cloned()
    }

    /// Maintenance périodique (timeouts)
    pub fn run_maintenance(&self) {
        let mut games = self.games.lock().unwrap();
        for (_, game) in games.iter_mut() {
            game.check_timeouts();
        }
    }
}

impl Default for GameManager {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod game_isolation_tests {
    use super::*;
    use crate::models::{CreateGameRequest, JoinGameRequest, PlayerType, SubmitActionRequest, PlayerAction};

    #[test]
    fn test_multiple_games_isolation() {
        let manager = GameManager::new();

        // Créer Game A
        let req_a = CreateGameRequest {
            name: "Game A".to_string(),
            max_players: 2,
            starting_chips: 1000,
            small_blind: 10,
            big_blind: 20,
        };
        let resp_a = manager.create_game(req_a).unwrap();
        let game_id_a = resp_a.game_id;

        // Créer Game B
        let req_b = CreateGameRequest {
            name: "Game B".to_string(),
            max_players: 2,
            starting_chips: 1000,
            small_blind: 10,
            big_blind: 20,
        };
        let resp_b = manager.create_game(req_b).unwrap();
        let game_id_b = resp_b.game_id;

        assert_ne!(game_id_a, game_id_b);

        // Rejoindre Game A (Alice)
        let join_a = JoinGameRequest {
            bot_name: "Alice".to_string(),
            player_type: PlayerType::Human,
            bot_secret: None,
        };
        let p_a = manager.join_game(game_id_a, join_a).unwrap();

        // Rejoindre Game B (Bob) - Same name, different game
        let join_b = JoinGameRequest {
            bot_name: "Bob".to_string(),
            player_type: PlayerType::Human,
            bot_secret: None,
        };
        let p_b = manager.join_game(game_id_b, join_b).unwrap();

        // Vérifier que Alice n'est PAS dans Game B
        let state_b = manager.get_game_state(game_id_b, &p_b.player_id).unwrap();
        assert!(state_b.players.iter().all(|p| p.id != p_a.player_id));

        // Vérifier que Bob n'est PAS dans Game A
        let state_a = manager.get_game_state(game_id_a, &p_a.player_id).unwrap();
        assert!(state_a.players.iter().all(|p| p.id != p_b.player_id));

        // Tenter d'utiliser token A dans Game B -> Doit échouer
        let action_request = SubmitActionRequest {
            auth_token: p_a.auth_token.clone(),
            action: PlayerAction::Fold,
        };
        let res = manager.submit_action(game_id_b, action_request);
        assert!(res.is_err()); // "Token invalide pour cette partie"
    }
}
