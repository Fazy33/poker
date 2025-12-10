use serde::{Deserialize, Serialize};
use uuid::Uuid;
use poker_engine::{PlayerAction as EngineAction, GamePhase, Card};

/// Identifiant unique d'une partie
pub type GameId = Uuid;

/// Identifiant unique d'un joueur/bot
pub type PlayerId = String;

/// Type de joueur
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlayerType {
    Human,
    Bot,
}

/// Requête pour créer une nouvelle partie
#[derive(Debug, Deserialize)]
pub struct CreateGameRequest {
    pub name: String,
    pub max_players: usize,
    pub starting_chips: u32,
    pub small_blind: u32,
    pub big_blind: u32,
}

/// Réponse après création d'une partie
#[derive(Debug, Serialize)]
pub struct CreateGameResponse {
    pub game_id: GameId,
    pub name: String,
}

/// Requête pour rejoindre une partie
#[derive(Debug, Deserialize)]
pub struct JoinGameRequest {
    pub bot_name: String,
    #[serde(default = "default_player_type")]
    pub player_type: PlayerType,
    #[serde(default)]
    #[allow(dead_code)] // Pour usage futur (authentification)
    pub bot_secret: Option<String>,
}

fn default_player_type() -> PlayerType {
    PlayerType::Bot
}

/// Réponse après avoir rejoint une partie
#[derive(Debug, Serialize)]
pub struct JoinGameResponse {
    pub player_id: PlayerId,
    pub game_id: GameId,
    pub position: usize,
    pub auth_token: String,  // Token JWT pour l'authentification
}

/// Action d'un joueur (format API)
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum PlayerAction {
    Fold,
    Check,
    Call,
    Raise { amount: u32 },
    AllIn,
}

impl From<PlayerAction> for EngineAction {
    fn from(action: PlayerAction) -> Self {
        match action {
            PlayerAction::Fold => EngineAction::Fold,
            PlayerAction::Check => EngineAction::Check,
            PlayerAction::Call => EngineAction::Call,
            PlayerAction::Raise { amount } => EngineAction::Raise(amount),
            PlayerAction::AllIn => EngineAction::AllIn,
        }
    }
}

/// Requête pour soumettre une action
#[derive(Debug, Deserialize)]
pub struct SubmitActionRequest {
    pub auth_token: String,  // Token JWT au lieu de player_id
    pub action: PlayerAction,
}

/// Réponse après soumission d'une action
#[derive(Debug, Serialize)]
pub struct SubmitActionResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Information sur un joueur (pour l'API)
#[derive(Debug, Serialize, Clone)]
pub struct PlayerInfo {
    pub id: PlayerId,
    pub name: String,
    pub chips: u32,
    pub current_bet: u32,
    pub status: String,
    pub player_type: PlayerType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards: Option<Vec<String>>, // Seulement pour le joueur lui-même
}

/// État du jeu/// Réponse d'état de jeu
#[derive(Debug, Serialize)]
pub struct GameStateResponse {
    pub game_id: GameId,
    pub phase: String,
    pub pot: u32,
    pub current_bet: u32,
    pub community_cards: Vec<String>,
    pub players: Vec<PlayerInfo>,
    pub current_player_id: Option<PlayerId>,
    pub your_player_id: Option<PlayerId>,
    pub your_chips: Option<u32>,
    pub your_cards: Option<Vec<String>>,
    pub valid_actions: Vec<String>,
    pub game_finished: bool,
    pub winner_id: Option<PlayerId>,
    pub winner_name: Option<String>,
    pub action_log: Option<Vec<String>>,
    pub last_hand_winner: Option<PlayerId>,
    pub last_hand_winner_name: Option<String>,
    pub last_hand_amount: Option<u32>,
    pub last_hand_description: Option<String>,
    pub last_hand_cards: Option<Vec<String>>,
}

/// Liste des parties disponibles
#[derive(Debug, Serialize)]
pub struct GameListResponse {
    pub games: Vec<GameSummary>,
}

/// Résumé d'une partie
#[derive(Debug, Serialize)]
pub struct GameSummary {
    pub game_id: GameId,
    pub name: String,
    pub player_count: usize,
    pub max_players: usize,
    pub phase: String,
    pub pot: u32,
}

/// Événement WebSocket pour les mises à jour en temps réel
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(dead_code)] // Pour usage futur (WebSocket)
pub enum GameEvent {
    GameUpdate {
        game_id: GameId,
        phase: String,
        pot: u32,
        community_cards: Vec<String>,
        current_player: Option<String>,
    },
    PlayerAction {
        game_id: GameId,
        player_name: String,
        action: String,
        amount: Option<u32>,
    },
    PlayerJoined {
        game_id: GameId,
        player_name: String,
    },
    GameStarted {
        game_id: GameId,
    },
    GameEnded {
        game_id: GameId,
        winner: String,
    },
}

/// Convertir une carte en string pour l'API
pub fn card_to_string(card: &Card) -> String {
    format!("{}", card)
}

/// Convertir une phase en string
pub fn phase_to_string(phase: &GamePhase) -> String {
    match phase {
        GamePhase::PreFlop => "preflop".to_string(),
        GamePhase::Flop => "flop".to_string(),
        GamePhase::Turn => "turn".to_string(),
        GamePhase::River => "river".to_string(),
        GamePhase::Showdown => "showdown".to_string(),
    }
}
