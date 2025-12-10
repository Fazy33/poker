use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// Secret pour signer les JWT (à mettre en variable d'environnement en production)
const JWT_SECRET: &[u8] = b"poker_secret_key_change_in_production";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub player_id: String,
    pub game_id: String,
    pub exp: i64, // Expiration timestamp
}

/// Créer un token JWT pour un joueur
pub fn create_token(player_id: &str, game_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        player_id: player_id.to_string(),
        game_id: game_id.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

/// Vérifier et décoder un token JWT
pub fn verify_token(token: &str) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Token expiré".to_string(),
        jsonwebtoken::errors::ErrorKind::InvalidToken => "Token invalide".to_string(),
        _ => format!("Erreur de validation du token: {}", e),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_verify_token() {
        let player_id = "test_player";
        let game_id = "test_game";

        let token = create_token(player_id, game_id).unwrap();
        let claims = verify_token(&token).unwrap();

        assert_eq!(claims.player_id, player_id);
        assert_eq!(claims.game_id, game_id);
    }

    #[test]
    fn test_invalid_token() {
        let result = verify_token("invalid_token");
        assert!(result.is_err());
    }
}
