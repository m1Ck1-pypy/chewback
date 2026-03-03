use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Обертка для UUID в параметрах пути
#[derive(ToSchema)]
pub struct UuidWrapper(pub Uuid);

/// Стандартный ответ с сообщением
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    /// Сообщение от сервера
    pub message: String,
}

/// Детали ошибки валидации
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ValidationError {
    /// Поле с ошибкой
    pub field: String,
    /// Описание ошибки
    pub message: String,
}

/// Расширенный ответ об ошибке
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    /// Тип ошибки
    pub error: String,
    /// Сообщение об ошибке
    pub message: String,
    /// Детали ошибки (опционально)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<ValidationError>>,
}