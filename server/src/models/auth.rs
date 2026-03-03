use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    /// Логин пользователя
    #[schema(example = "john_doe")]
    pub login: String,
    /// Пароль пользователя
    #[schema(example = "password123")]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    /// Данные пользователя
    pub user: super::user::UserResponse,
    /// Тип токена
    #[schema(example = "Bearer")]
    /// Время жизни токена в секундах
    pub token_type: String,
    #[schema(example = 86400)]
    pub expires_in: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponseWithTokens {
    /// Данные пользователя
    pub user: super::user::UserResponse,
    /// Тип токена
    #[schema(example = "Bearer")]
    pub token_type: String,
    /// Время жизни токена в секундах
    #[schema(example = 86400)]
    pub expires_in: i64,
    /// Access токен (JWT)
    #[schema(example = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...")]
    pub access_token: String,
    /// Refresh токен
    #[schema(example = "dGhpcyBpcyBhIHJlZnJlc2ggdG9rZW4...")]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenPair {
    /// Access токен (JWT)
    #[schema(example = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...")]
    pub access_token: String,
    /// Refresh токен
    #[schema(example = "dGhpcyBpcyBhIHJlZnJlc2ggdG9rZW4...")]
    pub refresh_token: String,
    /// Время жизни access токена в секундах
    #[schema(example = 86400)]
    pub expires_in: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    /// Refresh токен (опционально, если передан в cookie)
    #[schema(example = "dGhpcyBpcyBhIHJlZnJlc2ggdG9rZW4...")]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AccessTokenClaims {
    /// Subject (ID пользователя)
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub sub: String, // user_id
    /// Логин пользователя
    #[schema(example = "john_doe")]
    pub login: String,
    /// Issued at (timestamp)
    #[schema(example = 1705315800)]
    pub iat: i64,
    /// Expiration (timestamptz)
    #[schema(example = 1705402200)]
    pub exp: i64, // expiration
    /// Тип токена
    #[schema(example = "access")]
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenClaims {
    /// Subject (ID сессии)
    #[schema(example = "660e8400-e29b-41d4-a716-446655440001")]
    pub sub: String,
    /// ID пользователя
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub user_id: String,
    /// Issued at (timestamp)
    #[schema(example = 1705315800)]
    pub iat: i64,
    /// Expiration (timestamptz)
    #[schema(example = 1705402200)]
    pub exp: i64,
    /// Тип токена
    #[schema(example = "refresh")]
    pub token_type: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub refresh_token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
