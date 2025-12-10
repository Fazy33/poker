use actix_web::{web, HttpResponse, Result};
use crate::game_manager::GameManager;
use crate::models::*;
use uuid::Uuid;

/// POST /api/games - Créer une nouvelle partie
pub async fn create_game(
    game_manager: web::Data<GameManager>,
    req: web::Json<CreateGameRequest>,
) -> Result<HttpResponse> {
    match game_manager.create_game(req.into_inner()) {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }))),
    }
}

/// GET /api/games - Lister toutes les parties
pub async fn list_games(
    game_manager: web::Data<GameManager>,
) -> Result<HttpResponse> {
    let response = game_manager.list_games();
    Ok(HttpResponse::Ok().json(response))
}

/// POST /api/games/{id}/join - Rejoindre une partie
pub async fn join_game(
    game_manager: web::Data<GameManager>,
    game_id: web::Path<Uuid>,
    req: web::Json<JoinGameRequest>,
) -> Result<HttpResponse> {
    match game_manager.join_game(*game_id, req.into_inner()) {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }))),
    }
}

/// POST /api/games/{id}/start - Démarrer une partie
pub async fn start_game(
    game_manager: web::Data<GameManager>,
    game_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    match game_manager.start_game(*game_id) {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }))),
    }
}

/// POST /api/games/{id}/action - Soumettre une action
pub async fn submit_action(
    game_manager: web::Data<GameManager>,
    game_id: web::Path<Uuid>,
    req: web::Json<SubmitActionRequest>,
) -> Result<HttpResponse> {
    match game_manager.submit_action(*game_id, req.into_inner()) {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }))),
    }
}

/// GET /api/games/{id}/state - Obtenir l'état de la partie
pub async fn get_game_state(
    game_manager: web::Data<GameManager>,
    game_id: web::Path<Uuid>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse> {
    let player_id = query.get("player_id")
        .ok_or_else(|| actix_web::error::ErrorBadRequest("player_id requis"))?;

    match game_manager.get_game_state(*game_id, player_id) {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }))),
    }
}

/// Configuration des routes API
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/games", web::post().to(create_game))
            .route("/games", web::get().to(list_games))
            .route("/games/{id}/join", web::post().to(join_game))
            .route("/games/{id}/start", web::post().to(start_game))
            .route("/games/{id}/action", web::post().to(submit_action))
            .route("/games/{id}/state", web::get().to(get_game_state))
    );
}
