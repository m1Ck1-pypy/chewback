use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use tower_cookies::Cookies;

use crate::{
    database::AppState,
    errors::AppError,
    models::user::{User, UserRole},
    services::auth::jwt::TokenManager,
};

/// Извлечение авторизованнгого пользователя из запроса
pub struct AuthUser(pub User);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Получает cookies
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::Unauthorized("Failed to extract cookies".to_string()))?;

        let app_state = AppState::from_ref(state);

        let token = cookies
            .get(&app_state.config.cookie_name)
            .ok_or_else(|| AppError::Unauthorized("No auth token provided".to_string()))?
            .value()
            .to_string();

        let token_manager = TokenManager::new(app_state.config.clone());
        let claims = token_manager.validate_access_token(&token)?;

        let user = sqlx::query_as::<_, User>(
            "SELECT id, login, password_hash, role, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(&claims.sub)
        .fetch_optional(&app_state.db)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::UserNotFound("User not found".to_string()))?;

        Ok(AuthUser(user))
    }
}

/// Опциональное извлечение пользователя (для публичных роутов с доп. функциями для авторизованных)
pub struct MaybeAuthUser(pub Option<User>);

impl<S> FromRequestParts<S> for MaybeAuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let Ok(cookies) = Cookies::from_request_parts(parts, state).await else {
            return Ok(MaybeAuthUser(None));
        };

        let Some(token_cookie) = cookies.get(&app_state.config.cookie_name) else {
            return Ok(MaybeAuthUser(None));
        };

        let token = token_cookie.value().to_string();

        let token_manager = TokenManager::new(app_state.config.clone());
        let Ok(claims) = token_manager.validate_access_token(&token) else {
            return Ok(MaybeAuthUser(None));
        };

        let user = sqlx::query_as(
            "SELECT id, login, password_hash, role, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(&claims.sub)
        .fetch_optional(&app_state.db)
        .await
        .ok()
        .flatten();

        Ok(MaybeAuthUser(user))
    }
}

/// Extractor для проверки роли пользователя
/// Администратор имеет доступ ко всем маршрутам независимо от требуемой роли
pub struct RequireRole(pub User);

impl<S> FromRequestParts<S> for RequireRole
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_user = AuthUser::from_request_parts(parts, state).await?;
        RequireRole::check(&auth_user.0, UserRole::Admin)?;
        Ok(RequireRole(auth_user.0))
    }
}

impl RequireRole {
    /// Проверяет, что пользователь имеет требуемую роль (или является админом)
    pub fn check(user: &User, required_role: UserRole) -> Result<(), AppError> {
        if user.role == UserRole::Admin || user.role == required_role {
            Ok(())
        } else {
            Err(AppError::Forbidden(format!(
                "Required role: {:?}, your role: {:?}",
                required_role, user.role
            )))
        }
    }
}
