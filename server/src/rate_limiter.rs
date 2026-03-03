use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use governor::{
    Quota, RateLimiter,
    clock::DefaultClock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
};
use nonzero_ext::nonzero;
use std::{num::NonZeroU32, sync::Arc, time::Duration};

use crate::errors::AppError;

/// Конфигурация rate limiting
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Максимальное количество запросов
    pub max_requests: NonZeroU32,
    /// Период времени в секундах
    pub period_secs: NonZeroU32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            // 5 запросов в минуту для auth endpoints
            max_requests: nonzero!(5u32),
            period_secs: nonzero!(60u32),
        }
    }
}

/// Rate limiter state (in-memory, без ключа - глобальный для всех)
pub type RateLimitState = RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>;

/// Middleware состояние для rate limiting
#[derive(Clone)]
pub struct RateLimitMiddleware {
    limiter: Arc<RateLimitState>,
    message: String,
}

impl RateLimitMiddleware {
    /// Создать новый middleware с заданной конфигурацией
    pub fn new(config: RateLimitConfig, message: impl Into<String>) -> Self {
        let period =
            Duration::from_secs(config.period_secs.get() as u64) / config.max_requests.get();
        let quota = Quota::with_period(period)
            .unwrap()
            .allow_burst(config.max_requests);

        Self {
            limiter: Arc::new(RateLimiter::direct(quota)),
            message: message.into(),
        }
    }

    /// Создать middleware для login (5 запросов в минуту)
    pub fn login() -> Self {
        Self::new(
            RateLimitConfig {
                max_requests: nonzero!(5u32),
                period_secs: nonzero!(60u32),
            },
            "Too many login attempts. Please try again later.",
        )
    }

    /// Создать middleware для register (3 запроса в минуту)
    pub fn register() -> Self {
        Self::new(
            RateLimitConfig {
                max_requests: nonzero!(3u32),
                period_secs: nonzero!(60u32),
            },
            "Too many registration attempts. Please try again later.",
        )
    }

    /// Создать middleware для refresh token (10 запросов в минуту)
    pub fn refresh() -> Self {
        Self::new(
            RateLimitConfig {
                max_requests: nonzero!(10u32),
                period_secs: nonzero!(60u32),
            },
            "Too many token refresh attempts. Please try again later.",
        )
    }
}

/// Middleware функция для rate limiting
pub async fn rate_limit_middleware(
    State(middleware): State<RateLimitMiddleware>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let limiter = &middleware.limiter;

    // Проверяем, превышен ли лимит
    if limiter.check().is_err() {
        tracing::warn!("Rate limit exceeded");
        return Err(AppError::RateLimitExceeded(middleware.message.clone()));
    }

    Ok(next.run(request).await)
}

/// Extension trait для добавления rate limiting на роуты
#[allow(dead_code)]
pub trait RateLimitedRouter<S> {
    fn rate_limit(self, middleware: RateLimitMiddleware) -> Self;
}

impl<S> RateLimitedRouter<S> for axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn rate_limit(self, middleware: RateLimitMiddleware) -> Self {
        self.layer(axum::middleware::from_fn_with_state(
            middleware,
            rate_limit_middleware,
        ))
    }
}
