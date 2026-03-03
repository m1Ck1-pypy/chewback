use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Error, ToSchema)]
pub enum AppError {
    #[error("EnvVarError: {0}")]
    EnvVar(String),

    #[error("ParseIntError: {0}")]
    ParseInt(String),

    #[error("IoError: {0}")]
    Io(String),

    /// 500 Database error
    #[error("Database error")]
    DatabaseError(String),

    /// 404 User not found
    #[error("User not found")]
    UserNotFound(String),

    /// 409 User already exists
    #[error("User already exists")]
    UserAlreadyExists(String),

    /// 401 Invalid token
    #[error("Invalid token: {0}")]
    InvalidToken(String),

    /// 401 Unauthorized
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// 403 Forbidden
    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// 400 Invalid credentials
    #[error("Invalid credentials")]
    InvalidCredentials,

    /// 400 Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// 500 Internal error
    #[error("Internal server error")]
    InternalError(String),

    /// 429 Rate limit exceeded
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    /// 400 Password hash string invalid
    #[error("Password hash string invalid.")]
    PhcStringField(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type) = match &self {
            AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::Forbidden(_) => (StatusCode::FORBIDDEN, "Forbidden"),
            AppError::InvalidToken(_) => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::UserNotFound(_) => (StatusCode::NOT_FOUND, "Not Found"),
            AppError::UserAlreadyExists(_) => (StatusCode::CONFLICT, "Conflict"),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, "Bad Request"),
            AppError::EnvVar(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            AppError::ParseInt(_) => (StatusCode::BAD_REQUEST, "Bad Request"),
            AppError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            AppError::DatabaseError(_) => {
                tracing::error!(error = %self, "Database error occurred");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            AppError::InternalError(_) => {
                tracing::error!(error = %self, "Internal error occurred");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            AppError::RateLimitExceeded(_) => (StatusCode::TOO_MANY_REQUESTS, "Too Many Requests"),
            AppError::PhcStringField(err) => {
                tracing::error!(error = %self, "Password Hash error");
                (StatusCode::BAD_REQUEST, err.as_str())
            }
        };

        let body = Json(json!({
            "error": error_type,
            "message": self.to_string(),
        }));

        (status, body).into_response()
    }
}

impl From<dotenvy::Error> for AppError {
    fn from(err: dotenvy::Error) -> Self {
        AppError::EnvVar(err.to_string())
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::ParseInt(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        tracing::debug!(error = %err, "SQLx error occurred");
        match &err {
            sqlx::Error::RowNotFound => AppError::UserNotFound("User not found".to_string()),
            sqlx::Error::Database(db_err) => {
                if db_err.message().contains("unique") {
                    AppError::UserAlreadyExists("User already exists".to_string())
                } else {
                    AppError::DatabaseError(err.to_string())
                }
            }
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::InvalidToken(err.to_string())
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        match &err {
            argon2::password_hash::Error::PhcStringField => {
                AppError::PhcStringField("Password hash string invalid.".to_string())
            }
            _ => AppError::InternalError(err.to_string()),
        }
    }
}
