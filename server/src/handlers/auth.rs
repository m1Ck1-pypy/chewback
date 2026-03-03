use axum::{Json, extract::State};
use chrono::Utc;
use tower_cookies::{Cookie, Cookies, cookie::time};
use uuid::Uuid;

use crate::{
    database::AppState,
    errors::AppError,
    models::{api::*, auth::*, user::*},
    services::{AuthUser, TokenManager, hash_password, hash_refresh_token, verify_password},
};

/// Регистрация нового пользователя
///
/// Создает нового пользователя с указанным логином и паролем.
/// Автоматически авторизует пользователя и устанавливает cookies.
#[utoipa::path(
    post,
    path = "/register",
    tag = "Authentication",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = AuthResponseWithTokens),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 409, description = "User already exists", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
pub async fn register(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<AuthResponseWithTokens>, AppError> {
    if payload.login.len() < 3 {
        return Err(AppError::ValidationError(
            "Login must be  at least 3 characters".to_string(),
        ));
    }
    if payload.password.len() < 6 {
        return Err(AppError::ValidationError(
            "Password must be at least 6 characters".to_string(),
        ));
    }

    let password_hash = hash_password(&payload.password)?;

    let user_id = Uuid::new_v4().to_string();
    let now = Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO users (id, login, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, $3, 'user', $4, $5)
        "#,
        &user_id,
        &payload.login,
        &password_hash,
        now,
        now,
    )
    .execute(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_err) if db_err.message().contains("unique constraint") => {
            AppError::UserAlreadyExists(format!("Login '{}' already taken", payload.login))
        }
        _ => AppError::DatabaseError(e.to_string()),
    })?;

    let session_id = Uuid::new_v4().to_string();
    let token_pair =
        state
            .token_manager
            .create_token_pair(&user_id, &payload.login, &session_id)?;

    let refresh_hash = hash_refresh_token(&token_pair.refresh_token)?;
    let expires_at =
        Utc::now() + chrono::Duration::days(state.config.refresh_token_expiration_days);

    sqlx::query!(
        r#"
        INSERT INTO sessions (id, user_id, refresh_token_hash, expires_at)
        VALUES ($1, $2, $3, $4)
        "#,
        &session_id,
        &user_id,
        &refresh_hash,
        expires_at,
    )
    .execute(&state.db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let mut refresh_cookie = Cookie::new("refresh_token", token_pair.refresh_token.clone());
    refresh_cookie.set_http_only(true);
    refresh_cookie.set_secure(false);
    refresh_cookie.set_same_site(tower_cookies::cookie::SameSite::Lax);
    refresh_cookie.set_path("/");
    refresh_cookie.set_max_age(time::Duration::days(
        state.config.refresh_token_expiration_days,
    ));
    cookies.add(refresh_cookie);

    // Устанавливаем access token cookie
    let mut access_cookie = Cookie::new(
        state.config.cookie_name.clone(),
        token_pair.access_token.clone(),
    );
    access_cookie.set_http_only(true);
    access_cookie.set_secure(false);
    access_cookie.set_same_site(tower_cookies::cookie::SameSite::Lax);
    access_cookie.set_path("/");
    access_cookie.set_max_age(time::Duration::hours(state.config.jwt_expiration_hours));
    cookies.add(access_cookie);

    let user_response = UserResponse {
        id: user_id,
        login: payload.login,
        role: UserRole::User,
        created_at: now,
    };

    Ok(Json(AuthResponseWithTokens {
        user: user_response,
        token_type: "Bearer".to_string(),
        expires_in: token_pair.expires_in,
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
    }))
}

/// Вход в систему
///
/// Авторизует пользователя по логину и паролю.
/// Устанавливает cookies с access и refresh токенами.
#[utoipa::path(
    post,
    path = "/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponseWithTokens),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
pub async fn login(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponseWithTokens>, AppError> {
    let user: User = sqlx::query_as(
        "SELECT id, login, password_hash, role, created_at, updated_at FROM users WHERE login = $1",
    )
    .bind(&payload.login)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::InvalidCredentials)?;

    let valid = verify_password(&payload.password, &user.password_hash)?;

    if !valid {
        return Err(AppError::InvalidCredentials);
    }

    let session_id = Uuid::new_v4().to_string();
    let expires_at =
        Utc::now() + chrono::Duration::days(state.config.refresh_token_expiration_days);

    // let token_pair = state.token_manager.create_token_pair(&user.id, &user.login, &session_id)?;
    // let refresh_hash = hash_refresh_token(&token_pair.refresh_token)?;

    // 3. Параллельное выполнение: создание токенов и хэширование refresh token
    let (token_pair, refresh_hash) = tokio::try_join!(
        async {
            state
                .token_manager
                .create_token_pair(&user.id, &user.login, &session_id)
        },
        async {
            // Сначала создаем refresh token, потом хэшируем его
            let temp_token_manager = TokenManager::new(state.config.clone());
            let refresh_token =
                temp_token_manager.create_token_pair(&user.id, &user.login, &session_id)?;
            hash_refresh_token(&refresh_token.refresh_token)
        }
    )?;

    sqlx::query!(
        r#"
        INSERT INTO sessions (id, user_id, refresh_token_hash, expires_at)
        VALUES ($1, $2, $3, $4)
        "#,
        &session_id,
        &user.id,
        &refresh_hash,
        &expires_at,
    )
    .execute(&state.db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Устанавливаем cookies
    let mut cookie = Cookie::new(
        state.config.cookie_name.clone(),
        token_pair.access_token.clone(),
    );
    cookie.set_http_only(true);
    cookie.set_secure(false);
    cookie.set_same_site(tower_cookies::cookie::SameSite::Lax);
    cookie.set_path("/");
    cookie.set_max_age(time::Duration::hours(state.config.jwt_expiration_hours));
    cookies.add(cookie);

    let mut refresh_cookie = Cookie::new("refresh_token", token_pair.refresh_token.clone());
    refresh_cookie.set_http_only(true);
    refresh_cookie.set_secure(false);
    refresh_cookie.set_same_site(tower_cookies::cookie::SameSite::Lax);
    refresh_cookie.set_path("/");
    refresh_cookie.set_max_age(time::Duration::days(
        state.config.refresh_token_expiration_days,
    ));
    cookies.add(refresh_cookie);

    Ok(Json(AuthResponseWithTokens {
        user: user.into(),
        token_type: "Bearer".to_string(),
        expires_in: token_pair.expires_in,
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
    }))
}

/// Обновление access токена
///
/// Использует refresh token для получения новой пары токенов.
/// Требует refresh token в cookie или теле запроса.
#[utoipa::path(
    post,
    path = "/refresh",
    tag = "Authentication",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = AuthResponseWithTokens),
        (status = 401, description = "Invalid or expired refresh token", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
pub async fn refresh_token(
    State(state): State<AppState>,
    cookies: Cookies,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<AuthResponseWithTokens>, AppError> {
    // Получаем refresh token из cookie или body
    let refresh_token = cookies
        .get("refresh_token")
        .map(|c| c.value().to_string())
        .or(payload.refresh_token)
        .ok_or_else(|| AppError::Unauthorized("No refresh token provided".to_string()))?;

    // Валидируем refresh token
    let claims = state.token_manager.validate_refresh_token(&refresh_token)?;

    // Ищем сессию в БД
    let session = sqlx::query_as!(
        Session,
        "SELECT id, user_id, refresh_token_hash, expires_at, created_at FROM sessions WHERE id = $1",
        &claims.sub
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or_else(|| AppError::InvalidToken("Sesssion not found".to_string()))?;

    // Проверяем срок действия сессии
    if session.expires_at < Utc::now() {
        // Удаляем просроченную сессию
        sqlx::query!("DELETE FROM sessions WHERE id = $1", &session.id)
            .execute(&state.db)
            .await
            .ok();
        // Возвращаем ошибку о просроченном токене
        return Err(AppError::InvalidToken("Invalid refresh token".to_string()));
    }

    // Получает пользователя
    let user: User = sqlx::query_as(
        "SELECT id, login, password_hash, role, created_at, updated_at FROM users WHERE id = $1",
    )
    .bind(&session.user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or_else(|| AppError::UserNotFound("User not found".to_string()))?;

    // Удаляем старую сессию
    sqlx::query!("DELETE FROM sessions WHERE id = $1", &session.id)
        .execute(&state.db)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Создаем новую пару токенов
    let new_session_id = Uuid::new_v4().to_string();
    let new_tokens =
        state
            .token_manager
            .create_token_pair(&user.id, &user.login, &new_session_id)?;

    // Создаем новую сессию
    let new_refresh_hash = hash_refresh_token(&new_tokens.refresh_token)?;
    let expires_at =
        Utc::now() + chrono::Duration::days(state.config.refresh_token_expiration_days);

    sqlx::query!(
        r#"
        INSERT INTO sessions (id, user_id, refresh_token_hash, expires_at)
        VALUES ($1, $2, $3, $4)
        "#,
        &new_session_id,
        &user.id,
        &new_refresh_hash,
        expires_at,
    )
    .execute(&state.db)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Обновляем cookies
    let mut cookie = Cookie::new(
        state.config.cookie_name.clone(),
        new_tokens.access_token.clone(),
    );
    cookie.set_http_only(true);
    cookie.set_secure(false);
    cookie.set_same_site(tower_cookies::cookie::SameSite::Strict);
    cookie.set_path("/");
    cookie.set_max_age(time::Duration::hours(state.config.jwt_expiration_hours));
    cookies.add(cookie);

    let mut refresh_cookie = Cookie::new("refresh_token", new_tokens.refresh_token.clone());
    refresh_cookie.set_http_only(true);
    refresh_cookie.set_secure(false);
    refresh_cookie.set_same_site(tower_cookies::cookie::SameSite::Strict);
    refresh_cookie.set_path("/");
    refresh_cookie.set_max_age(time::Duration::days(
        state.config.refresh_token_expiration_days,
    ));
    cookies.add(refresh_cookie);

    Ok(Json(AuthResponseWithTokens {
        user: user.into(),
        token_type: "Bearer".to_string(),
        expires_in: new_tokens.expires_in,
        access_token: new_tokens.access_token,
        refresh_token: new_tokens.refresh_token,
    }))
}

/// Выход из системы
///
/// Удаляет сессию из базы данных и очищает cookies.
/// Требует авторизации.
#[utoipa::path(
    post,
    path = "/logout",
    tag = "Authentication",
    security(
        ("cookieAuth" = [])
    ),
    responses(
        (status = 200, description = "Logout successful", body = MessageResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
pub async fn logout(
    State(state): State<AppState>,
    cookies: Cookies,
) -> Result<Json<serde_json::Value>, AppError> {
    // Получаем токен из cookie для инвалидации сессии
    if let Some(_token_cookie) = cookies.get(&state.config.cookie_name) {
        // Пытаемся получить session_id из refresh token cookie
        if let Some(refresh_cookie) = cookies.get("refresh_token")
            && let Ok(claims) = state
                .token_manager
                .validate_refresh_token(refresh_cookie.value())
        {
            sqlx::query!("DELETE FROM sessions WHERE id = $1", &claims.sub)
                .execute(&state.db)
                .await
                .ok();
        }
    }

    // Удаляем cookies
    let mut cookie = Cookie::new(state.config.cookie_name.clone(), "");
    cookie.set_http_only(true);
    cookie.set_secure(false);
    cookie.set_same_site(tower_cookies::cookie::SameSite::Strict);
    cookie.set_path("/");
    cookie.set_max_age(time::Duration::seconds(0));
    cookies.remove(cookie);

    let mut refresh_cookie = Cookie::new("refresh_token", "");
    refresh_cookie.set_http_only(true);
    refresh_cookie.set_secure(false);
    refresh_cookie.set_same_site(tower_cookies::cookie::SameSite::Strict);
    refresh_cookie.set_path("/api/v1/auth/refresh");
    refresh_cookie.set_max_age(time::Duration::seconds(0));
    cookies.remove(refresh_cookie);

    Ok(Json(serde_json::json!({
        "message": "Successfully logged out"
    })))
}

/// Получить текущего пользователя (me)
///
/// Возвращает информацию об авторизованном пользователе.
/// Требует авторизации.
#[utoipa::path(
    get,
    path = "/me",
    tag = "Authentication",
    security(
        ("cookieAuth" = [])
    ),
    responses(
        (status = 200, description = "Current user info", body = UserResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
)]
pub async fn me(user: AuthUser) -> Result<Json<UserResponse>, AppError> {
    Ok(Json(user.0.into()))
}
