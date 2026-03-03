mod config;
mod database;
mod errors;
mod handlers;
mod models;
mod rate_limiter;
mod routes;
mod services;

use crate::{
    database::{AppState, init_db},
    routes::ApiRoutes,
};
use utoipa_axum::router::OpenApiRouter;

#[tokio::main]
async fn main() -> Result<(), errors::AppError> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().init();

    let config = config::Config::new()?;
    let pool = init_db().await?;
    let state = AppState::new(pool, &config).await;

    let listener = tokio::net::TcpListener::bind(&config.local_url).await?;

    let (router, openapi_app) = OpenApiRouter::new()
        .nest("/api/v1/", ApiRoutes::init_v1(state))
        .split_for_parts();

    let app = router.merge(
        utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi_app),
    );

    tracing::info!("Listening server on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}
