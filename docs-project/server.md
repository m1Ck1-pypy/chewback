# Серверная часть проекта Chewback

## Обзор

Серверная часть Chewback - это высокопроизводительный бэкенд на Rust с использованием фреймворка Axum. Сервер предоставляет REST API для аутентификации, управления пользователями и в будущем - для обмена сообщениями в реальном времени через WebSocket.

## Архитектура

### Технологический стек

- **Rust 2024**: Системный язык программирования
- **Axum 0.8**: Веб-фреймворк с поддержкой async/await
- **PostgreSQL 17**: Реляционная база данных
- **SQLx 0.8**: Асинхронный SQL toolkit с проверкой типов
- **JWT**: JSON Web Tokens для аутентификации
- **Argon2**: Алгоритм хеширования паролей
- **OpenAPI**: Документация API через utoipa

### Структура проекта

```
server/
├── src/
│   ├── handlers/          # Обработчики HTTP запросов
│   │   ├── auth.rs        # Обработчики аутентификации
│   │   └── mod.rs         # Экспорт обработчиков
│   ├── models/            # Модели данных
│   │   ├── api.rs         # Общие структуры API
│   │   ├── auth.rs        # Модели аутентификации
│   │   ├── mod.rs         # Экспорт моделей
│   │   └── user.rs        # Модели пользователей
│   ├── routes/            # Маршруты API
│   │   ├── auth/          # Маршруты аутентификации
│   │   │   └── mod.rs     # Экспорт маршрутов auth
│   │   ├── users/         # Маршруты пользователей
│   │   │   └── mod.rs     # Экспорт маршрутов users
│   │   └── mod.rs         # Объединение маршрутов
│   ├── services/          # Бизнес-логика
│   │   └── auth.rs        # Сервис аутентификации
│   ├── config.rs          # Конфигурация приложения
│   ├── database.rs        # Подключение к БД
│   ├── errors.rs          # Система ошибок
│   ├── main.rs            # Точка входа
│   └── rate_limiter.rs    # Rate limiting middleware
├── migrations/            # Миграции базы данных
│   ├── 20260301164621_create_users_table.sql
│   ├── 20260301165137_create_sessions_table.sql
│   └── 20260301165202_update_tables.sql
└── Cargo.toml            # Зависимости Rust
```

## Ключевые компоненты

### 1. Система аутентификации

#### Модели данных

```rust
// Пользователь
pub struct User {
    pub id: String,
    pub login: String,
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Сессия пользователя
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub refresh_token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
```

#### Типы ролей

```sql
CREATE TYPE user_role AS ENUM ('admin', 'user', 'guest');
```

#### JWT токены

- **Access Token**: Короткоживущий (15 минут), для доступа к API
- **Refresh Token**: Долгоживущий (7 дней), для обновления access токена
- **Хранение**: Refresh токены хешируются (Argon2) перед сохранением в БД

### 2. API эндпоинты

#### Аутентификация

- `POST /api/v1/auth/register` - Регистрация нового пользователя
- `POST /api/v1/auth/login` - Вход в систему
- `POST /api/v1/auth/refresh` - Обновление access токена
- `POST /api/v1/auth/logout` - Выход из системы
- `GET /api/v1/auth/me` - Получение данных текущего пользователя

#### Пользователи (в разработке)

- `GET /api/v1/users` - Список пользователей
- `GET /api/v1/users/{id}` - Данные конкретного пользователя

### 3. Безопасность

#### Хеширование паролей

```rust
pub async fn hash_password(password: &str) -> Result<String> {
    let config = Config::default();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        config,
    );

    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}
```

#### Валидация паролей

- Минимальная длина: 8 символов
- Максимальная длина: 72 символа (ограничение Argon2)
- Требуется хотя бы одна цифра и одна буква

#### Rate Limiting

Используется библиотека `governor` для защиты от брутфорс атак:

- 5 попыток входа в минуту
- 10 запросов регистрации в час
- 20 запросов обновления токена в день

### 4. База данных

#### Миграции

1. **Создание таблицы пользователей:**
   - Поля: id, login, password_hash, role, created_at, updated_at
   - Индекс по login для быстрого поиска

2. **Создание таблицы сессий:**
   - Поля: id, user_id, refresh_token_hash, expires_at, created_at
   - Внешний ключ на users.id с каскадным удалением
   - Индексы по user_id и refresh_token_hash

3. **Обновление временных меток:**
   - Конвертация TIMESTAMP в TIMESTAMPTZ
   - Хранение времени в UTC

#### Подключение к БД

```rust
pub async fn connect() -> Result<Pool<Postgres>> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPool::connect(&database_url).await
}
```

### 5. Обработка ошибок

#### Иерархия ошибок

```rust
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Authentication failed")]
    Unauthorized,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal server error")]
    Internal,
}
```

#### Middleware для ошибок

```rust
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::UserNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        Json(json!({ "error": message })).into_response()
    }
}
```

### 6. Middleware

#### Аутентификация

```rust
pub async fn auth_middleware(
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    Extension(pool): Extension<PgPool>,
) -> Result<(Extension<User>, Extension<String>), AppError> {
    let token = auth_header.token();
    let claims = verify_jwt(token)?;

    let user = get_user_by_id(&pool, &claims.sub).await?;

    Ok((Extension(user), Extension(token.to_string())))
}
```

#### CORS

```rust
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_credentials(true)
}

```

#### Логирование

```rust
pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "chewback_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
```

## Конфигурация

### Переменные окружения

```env
DATABASE_URL=postgres://user:password@localhost:5431/chewback
JWT_SECRET=your-secret-key-here-change-in-production
RUST_LOG=chewback_server=debug,tower_http=debug
```

### Структура конфигурации

```rust
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_access_expiry: i64,
    pub jwt_refresh_expiry: i64,
    pub server_host: String,
    pub server_port: u16,
}
```

## Производительность

### Асинхронность

- Все обработчики используют async/await
- Блокирующие операции вынесены в thread pool
- Connection pooling для БД

### Кэширование

- Подготовленные запросы SQLx
- Connection pool с настройкой размера
- В будущем: Redis для сессий и кэша

### Мониторинг

- Structured logging через tracing
- Метрики запросов через tower-http
- Health check эндпоинты

## Разработка

### Запуск в development

```bash
# Установка зависимостей
cargo build

# Запуск миграций
sqlx migrate run

# Запуск сервера
cargo watch -x run
```

### Тестирование

#### Unit тесты

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_password_hashing() {
        let password = "TestPassword123";
        let hash = hash_password(password).await.unwrap();

        assert_ne!(password, hash);
        assert!(verify_password(password, &hash).await.unwrap());
    }
}
```

#### Интеграционные тесты

```bash
# Запуск тестов с тестовой БД
DATABASE_URL=postgres://user:password@localhost:5431/chewback_test cargo test
```

### Документация API

#### OpenAPI спецификация

- Автоматическая генерация через utoipa
- Интерактивная документация через Swagger UI
- Доступ по адресу: `http://localhost:3000/swagger-ui/`

#### Генерация клиентских типов

```bash
# Генерация TypeScript типов
bun x openapi-typescript http://localhost:3000/api-docs/openapi.json -o ./src/bindings/types.ts
```

## Безопасность

### Защита данных

1. **Пароли:**
   - Хеширование Argon2id
   - Соль генерируется для каждого пользователя
   - Параметры настройки для баланса безопасности/производительности

2. **Токены:**
   - JWT с HMAC-SHA256
   - Короткое время жизни access токенов
   - Refresh токены хранятся в БД в хешированном виде

3. **База данных:**
   - Подготовленные запросы для защиты от SQL инъекций
   - Транзакции для атомарности операций
   - Индексы для производительности

### Защита от атак

- **SQL Injection**: Prepared statements через SQLx
- **XSS**: Валидация и санитизация входных данных
- **CSRF**: SameSite cookies + проверка origin
- **Brute Force**: Rate limiting через governor
- **Timing Attacks**: Константное время сравнения хешей

## Масштабирование

### Горизонтальное масштабирование

1. **Stateless серверы:**
   - Сессии хранятся в БД
   - JWT токены содержат всю необходимую информацию
   - Любой сервер может обработать любой запрос

2. **Балансировка нагрузки:**
   - Поддержка нескольких инстансов
   - Health check эндпоинты
   - Graceful shutdown

3. **Кэширование:**
   - Redis для сессий и частых запросов
   - CDN для статических файлов
   - In-memory кэш для горячих данных

### Вертикальное масштабирование

- Настройка размера connection pool
- Оптимизация запросов к БД
- Кэширование результатов запросов
- Асинхронная обработка фоновых задач

## Планы по развитию

### Ближайшие задачи

1. **WebSocket сервер:**
   - Реализация real-time сообщений
   - Broadcast каналы для чатов
   - Сохранение сообщений в БД

2. **Система сообщений:**
   - Модели для сообщений и чатов
   - Эндпоинты для истории сообщений
   - Поиск по сообщениям

3. **Уведомления:**
   - Push уведомления о новых сообщениях
   - Email подтверждение регистрации
   - Восстановление пароля

### Долгосрочные планы

1. **Микросервисная архитектура:**
   - Выделение сервиса аутентификации
   - Сервис сообщений
   - Сервис уведомлений

2. **Мониторинг и аналитика:**
   - Метрики использования
   - Логирование в централизованную систему
   - APM для отслеживания производительности

3. **Интеграции:**
   - OAuth2 провайдеры
   - Сторонние сервисы уведомлений
   - Файловое хранилище

## Заключение

Серверная часть Chewback представляет собой современный, безопасный и производительный бэкенд на Rust. Архитектура построена с учетом лучших практик разработки веб-приложений и обеспечивает надежную основу для реализации функций мессенджера.

**Ключевые преимущества:**

- Высокая производительность благодаря Rust и async/await
- Безопасность через современные алгоритмы и практики
- Масштабируемость через stateless архитектуру
- Полная документация API через OpenAPI
- Простота разработки и тестирования

Проект готов к реализации дополнительных функций и масштабированию по мере роста пользовательской базы.
