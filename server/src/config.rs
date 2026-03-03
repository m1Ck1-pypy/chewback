use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct Config {
    pub local_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub refresh_token_expiration_days: i64,
    pub cookie_name: String,
}

impl Config {
    pub fn new() -> Result<Self, AppError> {
        let local_url = dotenvy::var("LOCAL_URL")?;
        let jwt_secret = dotenvy::var("JWT_SECRET")?;
        let jwt_expiration_hours = dotenvy::var("JWT_EXPIRATION_HOURS")?.parse::<i64>()?;
        let refresh_token_expiration_days =
            dotenvy::var("REFRESH_TOKEN_EXPIRATION_DAYS")?.parse::<i64>()?;
        let cookie_name = dotenvy::var("COOKIE_NAME")?;

        Ok(Config {
            local_url,
            jwt_secret,
            jwt_expiration_hours,
            refresh_token_expiration_days,
            cookie_name,
        })
    }
}
