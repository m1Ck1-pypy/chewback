# Руководство разработчика Chewback

## Введение

Это руководство предназначено для разработчиков, работающих над проектом Chewback. Оно содержит информацию о структуре проекта, процессах разработки, стандартах кодирования и инструментах.

## Содержание

1. [Начало работы](#начало-работы)
2. [Архитектура проекта](#архитектура-проекта)
3. [Стандарты кодирования](#стандарты-кодирования)
4. [Процесс разработки](#процесс-разработки)
5. [Тестирование](#тестирование)
6. [Отладка](#отладка)
7. [Работа с Git](#работа-с-git)
8. [Полезные команды](#полезные-команды)

## Начало работы

### Предварительные требования

- **Rust** 1.70+ (`rustup update`)
- **Node.js** 18+ или **Bun** 1.0+
- **PostgreSQL** 17+ (или Docker)
- **Tauri CLI** (`cargo install tauri-cli`)
- **SQLx CLI** (`cargo install sqlx-cli`)

### Настройка окружения

1. **Клонирование репозитория:**
   ```bash
   git clone <repository-url>
   cd chewback
   ```

2. **Установка зависимостей:**
   ```bash
   # Фронтенд зависимости
   bun install
   
   # Rust зависимости (устанавливаются автоматически)
   cd server
   cargo build
   ```

3. **Настройка базы данных:**
   ```bash
   # Запуск PostgreSQL через Docker
   docker-compose up -d
   
   # Запуск миграций
   cd server
   sqlx migrate run
   
   # Проверка подключения
   sqlx database create
   ```

4. **Настройка переменных окружения:**
   ```bash
   # Создание .env файла в server/
   cp server/.env.example server/.env
   
   # Редактирование .env файла
   # Установите DATABASE_URL и JWT_SECRET
   ```

## Архитектура проекта

### Обзор архитектуры

Проект состоит из трех основных частей:

1. **Клиент (Frontend):** Tauri + SolidJS + TypeScript
2. **Десктопная оболочка (Tauri):** Rust + системные API
3. **Сервер (Backend):** Rust + Axum + PostgreSQL

### Поток данных

```
Пользователь → Tauri App → HTTP API → Сервер → PostgreSQL
       ↑          ↓           ↑         ↓
       └──────────┴──── Tauri ┴─────────┘
           команды         Store
```

### Структура директорий

```
chewback/
├── src/                    # Фронтенд (SolidJS)
│   ├── api/               # API клиенты и типы
│   ├── context/           # Контексты SolidJS
│   ├── pages/             # Страницы приложения
│   ├── hooks/             # Кастомные хуки
│   └── bindings/          # Сгенерированные типы
├── src-tauri/             # Tauri приложение
│   └── src/               # Rust код Tauri
├── server/                # Серверная часть
│   ├── src/
│   │   ├── handlers/      # Обработчики запросов
│   │   ├── models/        # Модели данных
│   │   ├── routes/        # Маршруты API
│   │   └── services/      # Бизнес-логика
│   └── migrations/        # Миграции БД
└── docs-project/          # Документация
```

## Стандарты кодирования

### Rust

#### Форматирование

```bash
# Использование rustfmt
cargo fmt

# Проверка стиля
cargo clippy -- -D warnings
```

#### Соглашения по именованию

- **Структуры и перечисления:** `PascalCase`
- **Функции и переменные:** `snake_case`
- **Константы:** `SCREAMING_SNAKE_CASE`
- **Модули:** `snake_case`

#### Пример структуры

```rust
// Хороший пример
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub login: String,
    pub role: UserRole,
}

impl User {
    pub fn new(id: String, login: String) -> Self {
        Self {
            id,
            login,
            role: UserRole::User,
        }
    }
    
    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }
}
```

### TypeScript/SolidJS

#### Форматирование

```bash
# Использование Prettier
bun run prettier --write "src/**/*.{ts,tsx}"
```

#### Соглашения по именованию

- **Компоненты:** `PascalCase` с суффиксом `.tsx`
- **Хуки:** `useCamelCase` с суффиксом `.ts`
- **Функции:** `camelCase`
- **Константы:** `SCREAMING_SNAKE_CASE`
- **Типы и интерфейсы:** `PascalCase`

#### Пример компонента

```typescript
// Хороший пример
interface UserCardProps {
  user: User;
  onSelect?: (user: User) => void;
}

export function UserCard(props: UserCardProps) {
  const [isHovered, setIsHovered] = createSignal(false);
  
  const handleClick = () => {
    props.onSelect?.(props.user);
  };
  
  return (
    <div
      class="user-card"
      classList={{ 'user-card--hovered': isHovered() }}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      onClick={handleClick}
    >
      <div class="user-card__avatar">
        {props.user.login.charAt(0).toUpperCase()}
      </div>
      <div class="user-card__info">
        <h3 class="user-card__name">{props.user.login}</h3>
        <p class="user-card__role">{props.user.role}</p>
      </div>
    </div>
  );
}
```

### SQL миграции

#### Соглашения по именованию

- **Файлы миграций:** `YYYYMMDDHHMMSS_description.sql`
- **Таблицы:** `snake_case` во множественном числе
- **Индексы:** `idx_table_column`
- **Внешние ключи:** `fk_child_parent`

#### Пример миграции

```sql
-- Хороший пример
CREATE TABLE IF NOT EXISTS messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sender_id UUID NOT NULL,
    receiver_id UUID NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (sender_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (receiver_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_messages_sender_id ON messages(sender_id);
CREATE INDEX idx_messages_receiver_id ON messages(receiver_id);
CREATE INDEX idx_messages_created_at ON messages(created_at DESC);
```

## Процесс разработки

### Рабочий процесс

1. **Создание ветки:**
   ```bash
   git checkout -b feature/название-фичи
   ```

2. **Разработка:**
   - Пишите код согласно стандартам
   - Добавляйте тесты
   - Обновляйте документацию

3. **Тестирование:**
   ```bash
   # Запуск всех тестов
   just test
   
   # Или по отдельности
   cd server && cargo test
   ```

4. **Создание коммита:**
   ```bash
   git add .
   git commit -m "feat: добавить новую функциональность"
   ```

5. **Открытие Pull Request:**
   - Опишите изменения
   - Укажите связанные issues
   - Добавьте скриншоты если нужно

### Соглашения по коммитам

Используйте Conventional Commits:

- `feat:` Новая функциональность
- `fix:` Исправление бага
- `docs:` Изменения в документации
- `style:` Форматирование, отсутствующие точки с запятой и т.д.
- `refactor:` Рефакторинг кода
- `test:` Добавление или исправление тестов
- `chore:` Изменения в процессе сборки или вспомогательных инструментах

### Code Review

#### Что проверять:

1. **Функциональность:**
   - Код работает как задумано
   - Нет регрессий
   - Обработка edge cases

2. **Качество кода:**
   - Соответствие стандартам
   - Читаемость и понятность
   - Отсутствие дублирования

3. **Безопасность:**
   - Нет уязвимостей
   - Правильная обработка данных
   - Валидация входных данных

4. **Тестирование:**
   - Достаточное покрытие тестами
   - Тесты проходят
   - Моки и стабы корректны

## Тестирование

### Unit тесты

#### Rust (сервер)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_user_creation() {
        let user = User::new("test-id".to_string(), "testuser".to_string());
        
        assert_eq!(user.login, "testuser");
        assert_eq!(user.role, UserRole::User);
        assert!(!user.is_admin());
    }
}
```

#### TypeScript (клиент)

```typescript
import { describe, it, expect } from 'vitest';
import { User } from './user';

describe('User', () => {
  it('should create user with correct properties', () => {
    const user = new User('test-id', 'testuser');
    
    expect(user.id).toBe('test-id');
    expect(user.login).toBe('testuser');
    expect(user.role).toBe('user');
  });
});
```

### Интеграционные тесты

#### Тестирование API

```rust
#[cfg(test)]
mod integration_tests {
    use axum::http::{StatusCode, header};
    use super::*;
    
    #[tokio::test]
    async fn test_register_endpoint() {
        let app = create_test_app().await;
        
        let response = app
            .post("/api/v1/auth/register")
            .json(&RegisterRequest {
                login: "testuser".to_string(),
                password: "TestPassword123".to_string(),
            })
            .send()
            .await;
        
        assert_eq!(response.status(), StatusCode::CREATED);
    }
}
```

### E2E тесты

```typescript
// Пример с Playwright
import { test, expect } from '@playwright/test';

test('user can login', async ({ page }) => {
  await page.goto('http://localhost:1420/login');
  
  await page.fill('input[name="login"]', 'testuser');
  await page.fill('input[name="password"]', 'TestPassword123');
  await page.click('button[type="submit"]');
  
  await expect(page).toHaveURL('http://localhost:1420/');
  await expect(page.locator('.user-greeting')).toContainText('Welcome, testuser');
});
```

## Отладка

### Клиентская отладка

1. **DevTools Tauri:**
   ```bash
   # Запуск с DevTools
   bun run tauri dev -- --devtools
   ```

2. **Логирование:**
   ```typescript
   // В клиентском коде
   console.debug('Debug message:', data);
   console.error('Error occurred:', error);
   ```

3. **React DevTools:**
   - Установите расширение для SolidJS
   - Используйте для инспекции компонентов

### Серверная отладка

1. **Логирование:**
   ```rust
   use tracing::{info, error, debug};
   
   #[tracing::instrument]
   async fn handle_request() -> Result<()> {
       info!("Starting request processing");
       debug!("Detailed debug information");
       
       if let Err(e) = some_operation() {
           error!("Operation failed: {}", e);
           return Err(e);
       }
       
       info!("Request completed successfully");
       Ok(())
   }
   ```

2. **Отладка в VS Code:**
   ```json
   // launch.json
   {
     "type": "lldb",
     "request": "launch",
     "name": "Debug Server",
     "program": "${workspaceFolder}/server/target/debug/server",
     "args": [],
     "cwd": "${workspaceFolder}/server"
   }
   ```

3. **Профилирование:**
   ```bash
   # Профилирование CPU
   cargo flamegraph --bin server
   
   # Профилирование памяти
   valgrind --tool=massif ./target/debug/server
   ```

### Отладка базы данных

1. **Просмотр данных:**
   ```bash
   docker compose exec db psql -U user -d chewback -c "SELECT * FROM users;"
   ```

2. **Логи запросов:**
   ```sql
   -- Включение логирования медленных запросов
   ALTER DATABASE chewback SET log_min_duration_statement = '100ms';
   ```

3. **Explain запросов:**
   ```sql
   EXPLAIN ANALYZE SELECT * FROM users WHERE login = 'testuser';
   ```

## Работа с Git

### Ветвление

Используем Git Flow:

- `main` - Продакшен версия
- `develop` - Разработка
- `feature/*` - Новая функциональность
- `bugfix/*` - Исправление багов
- `release/*` - Подготовка релиза

### Мерж стратегия

- **Feature ветки:** Squash and merge
- **Bugfix ветки:** Rebase and merge
- **Release ветки:** Merge commit

### .gitignore

Убедитесь что следующие файлы не коммитятся:

- `node_modules/`
- `target/`
- `dist/`
- `.env`
- `*.log`
- `*.pid`
- `*.swp`

## Полезные команды

### Разработка

```bash
# Запуск всего приложения
just run

# Запуск только сервера
just server

# Запуск только клиента
bun run tauri dev

# Генерация типов TypeScript
just typegen

# Открытие Swagger UI
just swagger
```

### Тестирование

```bash
# Все тесты
just test

# Тесты сервера
cd server && cargo test

# Тесты с покрытием
cd server && cargo tarpaulin

# Линтинг
just lint
```

### База данных

```bash
# Создание миграции
cd server && sqlx migrate add название_миграции

# Запуск миграций
cd server && sqlx migrate run

# Откат миграции
cd server && sqlx migrate revert

# Сброс базы данных
just db-reset
```

### Сборка

```bash
# Development сборка
bun run tauri build --debug

# Production сборка
bun run tauri build

# Сборка для конкретной платформы
bun run tauri build --target x86_64-pc-windows-msvc
```
