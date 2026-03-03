mod auth;
mod users;

use crate::database::AppState;
use axum::{Json, body::Body};
use tower_cookies::CookieManagerLayer;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnFailure, TraceLayer},
};
use utoipa_axum::{router::OpenApiRouter, routes};

pub struct ApiRoutes;

impl ApiRoutes {
    pub fn init_v1(state: AppState) -> OpenApiRouter {
        OpenApiRouter::new()
            .routes(routes!(health))
            .nest("/auth", auth::routes(&state))
            .nest("/users", users::routes(&state))
            .layer(CookieManagerLayer::new())
            .layer(CorsLayer::permissive())
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(tracing::Level::ERROR))
                    .on_request(|_: &axum::http::Request<Body>, _: &tracing::Span| {
                        tracing::event!(tracing::Level::INFO, "Started processing request:");
                    })
                    .on_response(
                        |response: &axum::http::Response<Body>,
                         latency: std::time::Duration,
                         _: &tracing::Span| {
                            let latency = format!("{latency:?}");
                            if response.status().is_success() {
                                tracing::event!(
                                    tracing::Level::INFO,
                                    latency,
                                    status = %response.status(),
                                    "Finished processing request:"
                                );
                            } else {
                                tracing::event!(
                                    tracing::Level::ERROR,
                                    latency,
                                    status = %response.status(),
                                    "Finished processing request:"
                                );
                            };
                        },
                    )
                    .on_failure(DefaultOnFailure::new().level(tracing::Level::ERROR)),
            )
            .with_state(state)
    }

    // pub fn init_v2(state: AppState) -> OpenApiRouter {
    //     OpenApiRouter::new()
    //         .routes(routes!(health))
    //         .nest("/auth", auth::auth_routes(&state))
    //         .with_state(state)
    // }
}

/// Проверка здоровья API
///
/// Возвращает текущий статус API, timestamp и версию
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "API работает нормально", body = String)
    )
)]
async fn health() -> Json<String> {
    Json("OK, Health check!".to_string())
}
