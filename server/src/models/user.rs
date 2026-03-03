use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgArgumentBuffer, PgValueRef, Postgres};
use sqlx::{Decode, Encode, Type, encode::IsNull};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    #[default]
    User,
    Admin,
    Guest,
}

impl Type<Postgres> for UserRole {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("user_role")
    }
}

impl<'r> Decode<'r, Postgres> for UserRole {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        Ok(match s {
            "admin" => UserRole::Admin,
            "user" => UserRole::User,
            "guest" => UserRole::Guest,
            _ => return Err(format!("Unknown role: {}", s).into()),
        })
    }
}

impl<'q> Encode<'q, Postgres> for UserRole {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, sqlx::error::BoxDynError> {
        let s = match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
            UserRole::Guest => "guest",
        };
        <&str as Encode<Postgres>>::encode(s, buf)
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub login: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    #[schema(example = "john_doe", min_length = 3)]
    pub login: String,
    #[schema(example = "secure_password123", min_length = 6)]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    #[schema(example = "john_doe_updated", min_length = 3)]
    pub login: Option<String>,
    #[schema(example = "new_secure_password456", min_length = 6)]
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRoleRequest {
    #[schema(example = "admin")]
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: String,
    #[schema(example = "john_doe")]
    pub login: String,
    #[schema(example = "user")]
    pub role: UserRole,
    #[schema(example = "2024-01-15T10:30:00Z")]
    pub created_at: DateTime<Utc>,
}
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            login: user.login,
            role: user.role,
            created_at: user.created_at,
        }
    }
}
