# Лучшие практики разработки - Chewback

## Введение

Этот документ описывает лучшие практики разработки для проекта Chewback. Следование этим практикам обеспечивает высокое качество кода, безопасность и поддерживаемость проекта.

## Общие принципы

### 1. Принцип единственной ответственности (SRP)
Каждый модуль, класс или функция должна иметь одну и только одну причину для изменения.

**Хорошо:**
```rust
// Отдельный класс для работы с аутентификацией
struct AuthService {
    user_repo: UserRepository,
    token_service: TokenService,
}

impl AuthService {
    async fn authenticate(&self, login: &str, password: &str) -> Result<AuthResponse> {
        // Логика аутентификации
    }
}
```

**Плохо:**
```rust
// Класс, который делает слишком много
struct UserManager {
    // Управляет пользователями, токенами, логированием и т.д.
}
```

### 2. Принцип открытости/закрытости (OCP)
Программные сущности должны быть открыты для расширения, но закрыты для модификации.

**Хорошо:**
```typescript
// Абстракция для отправки сообщений
interface MessageSender {
    send(message: Message): Promise<void>;
}

// Реализации для разных типов сообщений
class TextMessageSender implements MessageSender {
    async send(message: Message) {
        // Отправка текстового сообщения
    }
}

class FileMessageSender implements MessageSender {
    async send(message: Message) {
        // Отправка файла
    }
}
```

### 3. KISS (Keep It Simple, Stupid)
Делайте решения максимально простыми. Избегайте ненужной сложности.

**Хорошо:**
```rust
// Простая и понятная функция
fn validate_password(password: &str) -> bool {
    password.len() >= 8 && password.len() <= 72
}
```

**Плохо:**
```rust
// Излишне сложная валидация
fn validate_password(password: &str) -> bool {
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;
    let mut has_special = false;
    
    for c in password.chars() {
        // Сложная логика проверки
    }
    
    // Сложные условия
    has_upper && has_lower && has_digit && has_special && password.len() > 7
}
```

## Безопасность

### 1. Валидация входных данных
Всегда валидируйте входные данные на всех уровнях приложения.

**Rust (сервер):**
```rust
use validator::Validate;

#[derive(Debug, Validate)]
struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    login: String,
    
    #[validate(length(min = 8, max = 72))]
    password: String,
}

async fn register(
    Json(request): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // Валидация автоматически выполняется
    request.validate()?;
    
    // Обработка запроса
}
```

**TypeScript (клиент):**
```typescript
function validateLogin(login: string): ValidationResult {
    if (login.length < 3) {
        return { valid: false, error: 'Логин слишком короткий' };
    }
    
    if (login.length > 50) {
        return { valid: false, error: 'Логин слишком длинный' };
    }
    
    if (!/^[a-zA-Z0-9_]+$/.test(login)) {
        return { valid: false, error: 'Логин содержит недопустимые символы' };
    }
    
    return { valid: true };
}
```

### 2. Безопасное хранение паролей
Всегда используйте современные алгоритмы хеширования.

**Rust:**
```rust
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub async fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
```

### 3. Защита от SQL инъекций
Всегда используйте prepared statements.

**Rust (SQLx):**
```rust
// Хорошо: Prepared statement
let user = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE login = $1",
    login
)
.fetch_optional(&pool)
.await?;

// Плохо: Конкатенация строк (уязвимо к SQL injection)
let query = format!("SELECT * FROM users WHERE login = '{}'", login);
let user = sqlx::query(&query).fetch_optional(&pool).await?;
```

## Производительность

### 1. Асинхронное программирование
Используйте async/await для неблокирующих операций.

**Rust:**
```rust
// Хорошо: Асинхронные операции
async fn get_user_with_messages(user_id: &str) -> Result<UserWithMessages> {
    let user = get_user(user_id).await?;
    let messages = get_user_messages(user_id).await?;
    
    Ok(UserWithMessages { user, messages })
}

// Плохо: Блокирующие вызовы в async контексте
async fn get_user_with_messages_bad(user_id: &str) -> Result<UserWithMessages> {
    let user = std::thread::spawn(|| {
        // Блокирующий вызов
        get_user_blocking(user_id)
    }).await.unwrap()?;
    
    Ok(UserWithMessages { user, messages: vec![] })
}
```

### 2. Кэширование
Кэшируйте часто используемые данные.

**Rust (с использованием OnceCell):**
```rust
use once_cell::sync::Lazy;
use std::collections::HashMap;

static CONFIG_CACHE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

async fn get_config(key: &str) -> Result<String> {
    let mut cache = CONFIG_CACHE.lock().await;
    
    if let Some(value) = cache.get(key) {
        return Ok(value.clone());
    }
    
    // Загрузка из базы данных
    let value = load_config_from_db(key).await?;
    cache.insert(key.to_string(), value.clone());
    
    Ok(value)
}
```

### 3. Оптимизация запросов к БД
Избегайте N+1 проблем.

**Плохо:**
```rust
async fn get_users_with_messages_bad(pool: &PgPool) -> Result<Vec<UserWithMessages>> {
    let users = get_all_users(pool).await?;
    
    let mut result = Vec::new();
    for user in users {
        // N+1 запросов!
        let messages = get_user_messages(&user.id, pool).await?;
        result.push(UserWithMessages { user, messages });
    }
    
    Ok(result)
}
```

**Хорошо:**
```rust
async fn get_users_with_messages_good(pool: &PgPool) -> Result<Vec<UserWithMessages>> {
    // Один запрос с JOIN
    let users_with_messages = sqlx::query_as!(
        UserWithMessagesRow,
        r#"
        SELECT 
            u.id as user_id,
            u.login,
            m.id as message_id,
            m.content
        FROM users u
        LEFT JOIN messages m ON u.id = m.sender_id
        ORDER BY u.id, m.created_at
        "#
    )
    .fetch_all(pool)
    .await?;
    
    // Группировка результатов
    // ...
}
```

## Качество кода

### 1. Тестирование
Пишите тесты для критической функциональности.

**Rust (unit тесты):**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_password_hashing() {
        let password = "TestPassword123";
        let hash = hash_password(password).await.unwrap();
        
        // Проверяем что пароль не хранится в открытом виде
        assert_ne!(password, hash);
        
        // Проверяем что верификация работает
        assert!(verify_password(password, &hash).await.unwrap());
        assert!(!verify_password("WrongPassword", &hash).await.unwrap());
    }
    
    #[test]
    fn test_validation() {
        let valid_request = RegisterRequest {
            login: "validuser".to_string(),
            password: "ValidPass123".to_string(),
        };
        
        assert!(valid_request.validate().is_ok());
        
        let invalid_request = RegisterRequest {
            login: "a".to_string(),  // Слишком короткий
            password: "short".to_string(),  // Слишком короткий
        };
        
        assert!(invalid_request.validate().is_err());
    }
}
```

**TypeScript (компонентные тесты):**
```typescript
import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@solidjs/testing-library';
import { LoginForm } from './LoginForm';

describe('LoginForm', () => {
    it('валидирует форму', () => {
        render(() => <LoginForm />);
        
        const submitButton = screen.getByRole('button', { name: /войти/i });
        fireEvent.click(submitButton);
        
        // Проверяем сообщения об ошибках
        expect(screen.getByText(/введите логин/i)).toBeInTheDocument();
        expect(screen.getByText(/введите пароль/i)).toBeInTheDocument();
    });
    
    it('отправляет форму при валидных данных', async () => {
        const mockOnSubmit = vi.fn();
        render(() => <LoginForm onSubmit={mockOnSubmit} />);
        
        const loginInput = screen.getByLabelText(/логин/i);
        const passwordInput = screen.getByLabelText(/пароль/i);
        const submitButton = screen.getByRole('button', { name: /войти/i });
        
        fireEvent.input(loginInput, { target: { value: 'testuser' } });
        fireEvent.input(passwordInput, { target: { value: 'Test123' } });
        fireEvent.click(submitButton);
        
        expect(mockOnSubmit).toHaveBeenCalledWith({
            login: 'testuser',
            password: 'Test123',
        });
    });
});
```

### 2. Обработка ошибок
Всегда обрабатывайте ошибки явно.

**Rust:**
```rust
// Хорошо: Явная обработка ошибок
async fn process_user_request(request: UserRequest) -> Result<UserResponse> {
    let user = match get_user(&request.user_id).await {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return Err(AppError::UserNotFound(request.user_id));
        }
        Err(e) => {
            tracing::error!("Failed to get user: {}", e);
            return Err(AppError::DatabaseError);
        }
    };
    
    // Дальнейшая обработка
    Ok(UserResponse { user })
}

// Плохо: Неявная обработка (unwrap)
async fn process_user_request_bad(request: UserRequest) -> UserResponse {
    let user = get_user(&request.user_id).await.unwrap(); // Может упасть!
    UserResponse { user }
}
```

**TypeScript:**
```typescript
// Хорошо: Обработка ошибок с типами
async function fetchUser(userId: string): Promise<Result<User, ApiError>> {
    try {
        const response = await api.get<User>(`/users/${userId}`);
        return { success: true, data: response.data };
    } catch (error) {
        if (error instanceof ApiError) {
            return { success: false, error };
        }
        
        // Неожиданная ошибка
        console.error('Unexpected error:', error);
        return { 
            success: false, 
            error: new ApiError('Неизвестная ошибка', 500) 
        };
    }
}

// Использование
const result = await fetchUser('123');
if (result.success) {
    // Работаем с данными
    console.log('User:', result.data);
} else {
    // Обрабатываем ошибку
    showError(result.error.message);
}
```

### 3. Логирование
Используйте структурированное логирование.

**Rust:**
```rust
use tracing::{info, error, warn, debug, instrument};

#[instrument(skip(pool))]
async fn authenticate_user(
    login: &str,
    password: &str,
    pool: &PgPool,
) -> Result<AuthResponse> {
    info!("Authenticating user: {}", login);
    
    let user = match get_user_by_login(login, pool).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            warn!("User not found: {}", login);
            return Err(AppError::InvalidCredentials);
        }
        Err(e) => {
            error!("Database error while fetching user: {}", e);
            return Err(AppError::DatabaseError);
        }
    };
    
    debug!("User found, verifying password");
    
    if !verify_password(password, &user.password_hash).await? {
        warn!("Invalid password for user: {}", login);
        return Err(AppError::InvalidCredentials);
    }
    
    info!("User authenticated successfully: {}", login);
    
    // Создание токенов и ответа
    Ok(create_auth_response(user).await?)
}
```

## Работа с Git

### 1. Коммиты
Следуйте Conventional Commits.

```
feat: добавить поддержку групповых чатов
fix: исправить утечку памяти в WebSocket соединениях
docs: обновить документацию API
style: форматировать код согласно rustfmt
refactor: переработать систему аутентификации
test: добавить тесты для end-to-end шифрования
chore: обновить зависимости
```

### 2. Ветвление
Используйте feature branches.

```bash
# Создание ветки для новой фичи
git checkout -b feat/add-group-chats

# Регулярные коммиты
git add .
git commit -m "feat: добавить модели для групповых чатов"

# Синхронизация с основной веткой
git fetch origin
git rebase origin/main

# Создание Pull Request
git push origin feat/add-group-chats
```

### 3. Code Review
Проводите code review для всех изменений.

**Что проверять:**
- Соответствие стандартам кодирования
- Наличие тестов
- Обработка ошибок
- Безопасность
- Производительность
- Читаемость кода

## Документация

### 1. Документация кода
Документируйте публичные API.

**Rust:**
```rust
/// Сервис для управления аутентификацией пользователей.
///
/// # Пример
/// ```
/// let auth_service = AuthService::new(pool);
/// let result = auth_service.authenticate("user", "pass").await;
/// ```
pub struct AuthService {
    user_repo: UserRepository,
    token_service: TokenService,
}

impl AuthService {
    /// Аутентифицирует пользователя по логину и паролю.
    ///
    /// # Аргументы
    /// * `login` - Логин пользователя
    /// * `password` - Пароль пользователя
    ///
    /// # Возвращает
    /// `AuthResponse` с токенами и данными пользователя при успешной аутентификации.
    ///
    /// # Ошибки
    /// Возвращает `AppError::InvalidCredentials` при неверных учетных данных.
    pub async fn authenticate(
        &self,
        login: &str,
        password: &str,
    ) -> Result<AuthResponse> {
        // Реализация
    }
}
```

**TypeScript:**
```typescript
/**
 * Сервис для работы с сообщениями.
 * 
 * @example
 * ```typescript
 * const messageService = new MessageService();
 * const messages = await messageService.getConversation('user123');
 * ```
 */
class MessageService {
    /**
     * Получает историю сообщений для диалога.
     * 
     * @param userId - ID пользователя для диалога
     * @param options - Опции пагинации
     * @returns Массив сообщений
     * 
     * @throws {ApiError} Если произошла ошибка API
     */
    async getConversation(
        userId: string,
        options?: PaginationOptions
    ): Promise<Message[]> {
        // Реализация
    }
}
```

### 2. Документация API
Используйте OpenAPI для документации REST API.

```rust
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::register,
        auth::login,
        auth::refresh,
        auth::logout,
        auth::get_me,
    ),
    components(
        schemas(RegisterRequest, LoginRequest, AuthResponse, User)
    ),
    tags(
        (name = "auth", description = "Аутентификация пользователей")
    )
)]
struct ApiDoc;

// Автоматическая генерация Swagger UI
app = app.merge(ApiDoc::openapi());
```

## Заключение

Следование этим лучшим практикам обеспечит:

1. **Надежность:** Минимизация ошибок и с
