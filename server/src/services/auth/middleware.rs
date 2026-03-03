use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use tower_cookies::Cookies;

use crate::{database::AppState, errors::AppError, services::auth::jwt::TokenManager};

pub async fn auth_middleware(
    state: State<AppState>,
    cookies: Cookies,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = cookies
        .get(&state.config.cookie_name)
        .map(|c| c.value().to_string())
        .ok_or_else(|| AppError::Unauthorized("Not auth token provided".to_string()))?;

    let token_manager = TokenManager::new(state.config.clone());
    let _access_claims = token_manager.validate_access_token(&token)?;

    Ok(next.run(request).await)
}
