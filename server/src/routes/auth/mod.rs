use axum::middleware;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    database::AppState, handlers::auth, rate_limiter::RateLimitMiddleware,
    services::auth_middleware,
};

pub fn routes(state: &AppState) -> OpenApiRouter<AppState> {
    let protected_routes = OpenApiRouter::new()
        .routes(routes!(auth::logout, auth::me))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // Public routes с rate limiting
    let login_route = OpenApiRouter::new().routes(routes!(auth::login)).layer(
        axum::middleware::from_fn_with_state(
            RateLimitMiddleware::login(),
            crate::rate_limiter::rate_limit_middleware,
        ),
    );

    let register_route = OpenApiRouter::new().routes(routes!(auth::register)).layer(
        axum::middleware::from_fn_with_state(
            RateLimitMiddleware::register(),
            crate::rate_limiter::rate_limit_middleware,
        ),
    );

    let refresh_route = OpenApiRouter::new()
        .routes(routes!(auth::refresh_token))
        .layer(axum::middleware::from_fn_with_state(
            RateLimitMiddleware::refresh(),
            crate::rate_limiter::rate_limit_middleware,
        ));

    let public_routes = OpenApiRouter::new()
        .merge(login_route)
        .merge(register_route)
        .merge(refresh_route);

    OpenApiRouter::new()
        .merge(protected_routes)
        .merge(public_routes)
}
