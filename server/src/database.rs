use crate::{config::Config, errors::AppError, services::TokenManager};
use axum::extract::FromRef;
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
    pub token_manager: TokenManager,
}

impl AppState {
    pub async fn new(db: PgPool, config: &Config) -> Self {
        AppState {
            db,
            config: config.clone(),
            token_manager: TokenManager::new(config.clone()),
        }
    }
}

// 2. Implement FromRef ONLY for the inner types
impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

impl FromRef<AppState> for TokenManager {
    fn from_ref(state: &AppState) -> Self {
        state.token_manager.clone()
    }
}

pub async fn init_db() -> Result<PgPool, AppError> {
    let database_url = dotenvy::var("DATABASE_URL")?;
    // let pool = sqlx::PgPool::connect(&database_url)
    //     .await
    //     .expect("Failed to connect to database");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .idle_timeout(std::time::Duration::from_secs(300))
        .max_lifetime(std::time::Duration::from_secs(1800))
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
        .ok();

    tracing::info!("✅ Database connected and migrations applied");
    Ok(pool)
}
