use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::{
    config::Config,
    errors::AppError,
    models::auth::{AccessTokenClaims, RefreshTokenClaims, TokenPair},
};

#[derive(Debug, Clone)]
pub struct TokenManager {
    config: Config,
}
impl TokenManager {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn create_token_pair(
        &self,
        user_id: &str,
        login: &str,
        session_id: &str,
    ) -> Result<TokenPair, AppError> {
        let access_token = self.create_access_token(user_id, login)?;
        let refresh_token = self.create_refresh_token(user_id, session_id)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            expires_in: self.config.jwt_expiration_hours * 3600,
        })
    }

    fn create_access_token(&self, user_id: &str, login: &str) -> Result<String, AppError> {
        let now = Utc::now();
        let expires_at = now + Duration::hours(self.config.jwt_expiration_hours);

        let claims = AccessTokenClaims {
            sub: user_id.to_string(),
            login: login.to_string(),
            iat: now.timestamp(),
            exp: expires_at.timestamp(),
            token_type: "access".to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )?;
        Ok(token)
    }

    fn create_refresh_token(&self, user_id: &str, session_id: &str) -> Result<String, AppError> {
        // let session_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = now + Duration::days(self.config.refresh_token_expiration_days);

        let claims = RefreshTokenClaims {
            sub: session_id.to_string(),
            user_id: user_id.to_string(),
            iat: now.timestamp(),
            exp: expires_at.timestamp(),
            token_type: "refresh".to_string(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )?;
        Ok(token)
    }

    pub fn validate_refresh_token(&self, token: &str) -> Result<RefreshTokenClaims, AppError> {
        let validation = Validation::default();

        let token_data = decode::<RefreshTokenClaims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &validation,
        )?;

        if token_data.claims.token_type != "refresh" {
            return Err(AppError::InvalidToken("Invalid token type".to_string()));
        }

        Ok(token_data.claims)
    }

    pub fn validate_access_token(&self, token: &str) -> Result<AccessTokenClaims, AppError> {
        let validation = Validation::default();

        let token_data = decode::<AccessTokenClaims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &validation,
        )?;

        if token_data.claims.token_type != "access" {
            return Err(AppError::InvalidToken("Invalid token type".to_string()));
        }

        Ok(token_data.claims)
    }

    pub fn _config(&self) -> &Config {
        &self.config
    }
}
