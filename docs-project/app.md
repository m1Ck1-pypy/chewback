# Tauri часть проекта Chewback

## Обзор

Tauri часть проекта Chewback отвечает за десктопную оболочку приложения, обеспечивая нативный доступ к операционной системе, безопасное хранение данных и мост между фронтендом на SolidJS и бэкендом на Rust.

## Архитектура

### Технологический стек

- **Tauri 2.0**: Фреймворк для создания десктопных приложений
- **Rust**: Системный язык для нативных операций
- **Keyring**: Безопасное хранение чувствительных данных
- **Reqwest**: HTTP клиент для взаимодействия с API
- **Serde**: Сериализация/десериализация данных
- **Tauri Store**: Локальное хранилище данных приложения

### Структура проекта

```
src-tauri/
├── src/
│   ├── lib.rs            # Основная логика Tauri
│   └── main.rs           # Точка входа
├── capabilities/         # Разрешения Tauri
├── gen/                 # Сгенерированный код
├── icons/               # Иконки приложения
├── Cargo.toml           # Зависимости Rust
├── tauri.conf.json      # Конфигурация Tauri
└── build.rs             # Скрипт сборки
```

## Ключевые компоненты

### 1. Система команд (Commands)

#### API запросы

```rust
#[tauri::command]
async fn api_request(
    method: String,
    endpoint: String,
    body: Option<Value>,
    headers: Option<HashMap<String, String>>,
) -> Result<Value, String> {
    // Реализация HTTP клиента
}
```

**Особенности:**

- Использует статический HTTP клиент с поддержкой cookies
- Автоматически добавляет Content-Type заголовок
- Обрабатывает ошибки сети и парсинга JSON
- Возвращает ошибки в формате "HTTP {status}: {message}"

#### Управление токенами

```rust
#[tauri::command]
async fn save_refresh_token(_app_handle: tauri::AppHandle, token: String) -> Result<(), String> {
    // Сохранение в системный keyring
}

#[tauri::command]
async fn get_refresh_token(_app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    // Получение из системного keyring
}

#[tauri::command]
async fn clear_refresh_token(_app_handle: tauri::AppHandle) -> Result<(), String> {
    // Удаление из системного keyring
}
```

#### Управление данными пользователя

```rust
#[tauri::command]
async fn save_user_data(app_handle: tauri::AppHandle, user_data: Value) -> Result<(), String> {
    // Сохранение в Tauri Store
}

#[tauri::command]
async fn get_user_data(app_handle: tauri::AppHandle) -> Result<Option<Value>, String> {
    // Получение из Tauri Store
}

#[tauri::command]
async fn clear_auth_data(app_handle: tauri::AppHandle) -> Result<(), String> {
    // Очистка всех данных авторизации
}
```

### 2. Безопасное хранение данных

#### Системный Keyring

Используется для хранения refresh токенов:

- **Windows**: Credential Manager
- **macOS**: Keychain
- **Linux**: Secret Service (GNOME Keyring/KWallet)

**Преимущества:**

- Данные защищены операционной системой
- Изолированы от файловой системы приложения
- Поддерживаются стандартные политики безопасности

#### Tauri Store

Используется для хранения данных пользователя:

- **Формат**: JSON файл (store.json)
- **Шифрование**: Встроенное шифрование Tauri
- **Расположение**: Директория данных приложения

**Хранимые данные:**

- Информация о пользователе (id, login, role)
- Настройки приложения
- Кэшированные данные

### 3. HTTP клиент

#### Конфигурация

```rust
static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .cookie_store(true) // Включаем поддержку cookies
        .build()
        .expect("Failed to create HTTP client")
});
```

**Особенности:**

- Статический клиент для повторного использования соединений
- Поддержка cookies для поддержания сессии
- Автоматическое управление соединениями

#### URL конфигурация

```rust
#[cfg(debug_assertions)]
const SERVER_URL: &str = "http://localhost:3000/api/v1/";

#[cfg(not(debug_assertions))]
const SERVER_URL: &str = "http://localhost:3000/api/v1/"; // Заменить на продакшен URL
```

### 4. Конфигурация приложения

#### tauri.conf.json

```json
{
  "productName": "chewback",
  "version": "0.1.0",
  "identifier": "com.kozar.chewback",
  "build": {
    "beforeDevCommand": "bun run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "bun run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "chewback",
        "width": 800,
        "height": 600
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

## Поток данных

### HTTP запросы

```
Frontend (SolidJS) → Tauri Command → Rust HTTP Client → Backend Server
       ↑                    ↑                ↑                ↑
       └── Response ←───────┴── Response ←───┴── Response ←───┘
```

### Хранение данных

```
Frontend → Tauri Command → Keyring/Store → Операционная система
   ↑            ↑              ↑                  ↑
   └── Data ←───┴── Data ←─────┴── Data ←─────────┘
```

## Безопасность

### Защита данных

1. **Refresh токены:**
   - Хранятся в системном keyring
   - Защищены операционной системой
   - Не доступны из файловой системы

2. **Данные пользователя:**
   - Хранятся в зашифрованном Store
   - Локально на устройстве пользователя
   - Очищаются при выходе из системы

3. **HTTP коммуникация:**
   - Все запросы проходят через Rust код
   - Поддержка HTTPS (в production)
   - Валидация ответов сервера

### Content Security Policy

В development режиме CSP отключен для удобства разработки. В production необходимо настроить:

```json
"security": {
  "csp": "default-src 'self'; connect-src 'self' http://your-api.com"
}
```

## Производительность

### Оптимизации

1. **Статический HTTP клиент:**
   - Повторное использование соединений
   - Connection pooling
   - Кэширование DNS запросов

2. **Ленивая инициализация:**
   - Keyring инициализируется только при необходимости
   - Store загружается по требованию
   - Плагины активируются при первом использовании

3. **Минимизация копирования:**
   - Передача данных по ссылкам
   - Использование Cow для строк
   - Буферизация ввода/вывода

### Мониторинг

- Логирование через tauri-plugin-log (только в dev режиме)
- Обработка и логирование ошибок команд
- Тайминги выполнения операций

## Разработка

### Настройка окружения

```bash
# Установка Tauri CLI
cargo install tauri-cli

# Запуск в development режиме
bun run tauri dev

# Сборка для production
bun run tauri build
```

### Структура команд

#### Правила именования:

- Команды: snake_case
- Параметры: snake_case
- Возвращаемые типы: Result<T, String>

#### Шаблон команды:

```rust
#[tauri::command]
async fn command_name(
    app_handle: tauri::AppHandle, // Для доступа к Store и плагинам
    param1: Type1,                // Обязательные параметры
    param2: Option<Type2>,        // Опциональные параметры
) -> Result<ReturnType, String> {
    // Валидация параметров
    // Бизнес-логика
    // Обработка ошибок
    // Возврат результата
}
```

### Тестирование

#### Unit тесты:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_request() {
        // Тестирование HTTP запросов
    }

    #[test]
    fn test_keyring_operations() {
        // Тестирование работы с keyring
    }
}
```

#### Интеграционные тесты:

- Тестирование взаимодействия с фронтендом
- Тестирование работы плагинов
- Тестирование конфигурации сборки

## Плагины

### Используемые плагины

1. **tauri-plugin-store:**
   - Локальное хранилище данных
   - Шифрование на лету
   - Автоматическая сериализация

2. **tauri-plugin-opener:**
   - Открытие внешних ссылок
   - Запуск системных приложений
   - Работа с файлами

3. **tauri-plugin-log:**
   - Структурированное логирование
   - Разные уровни логирования
   - Ротация логов

### Конфигурация плагинов

```rust
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Инициализация Store
            let _ = app.store("store.json")?;

            // Настройка логирования в dev режиме
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![/* команды */])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Сборка и дистрибуция

### Поддерживаемые платформы

- **Windows**: .exe установщик
- **macOS**: .app bundle
- **Linux**: .AppImage, .deb, .rpm

### Конфигурация сборки

```bash
# Сборка для текущей платформы
bun run tauri build

# Сборка для конкретной платформы
bun run tauri build --target x86_64-pc-windows-msvc
bun run tauri build --target aarch64-apple-darwin
bun run tauri build --target x86_64-unknown-linux-gnu
```

### Подписание кода

Для дистрибуции в магазинах приложений необходимо:

1. **Windows:** Сертификат код-сайнинга
2. **macOS:** Apple Developer Certificate
3. **Linux:** GPG подпись для репозиториев

## Планы по развитию

### Ближайшие задачи

1. **Уведомления:**
   - Системные уведомления о новых сообщениях
   - Badge на иконке приложения
   - Звуковые уведомления

2. **Интеграция с ОС:**
   - Глобальные горячие клавиши
   - Интеграция с меню ОС
   - Автозапуск при старте системы

3. **Работа с файлами:**
   - Отправка файлов через диалог выбора
   - Предпросмотр изображений
   - Сохранение вложений

### Долгосрочные планы

1. **Нативные функции:**
   - Скриншотинг
   - Запись экрана
   - Голосовые сообщения

2. **Производительность:**
   - Предзагрузка данных
   - Кэширование ресурсов
   - Оптимизация использования памяти

3. **Безопасность:**
   - Поддержка аппаратных ключей
   - Двухфакторная аутентификация
   - Шифрование локальных данных

## Отладка и мониторинг

### Dev Tools

В development режиме доступны:

- Консоль разработчика (F12)
- Инспектор элементов
- Сетевые запросы
- Производительность

### Логирование

Уровни логирования:

- `error`: Критические ошибки
- `warn`: Предупреждения
- `info`: Информационные сообщения
- `debug`: Отладочная информация
- `trace`: Детальная трассировка

### Профилирование

Инструменты для анализа производительности:

- `cargo flamegraph` для профилирования Rust кода
- Chrome DevTools для профилирования фронтенда
- `perf` на Linux для системного профилирования

## Заключение

Tauri часть Chewback обеспечивает надежную и безопасную десктопную оболочку для мессенджера. Архитектура построена с учетом современных практик разработки десктопных приложений и обеспечивает:

- **Безопасность**: Защищенное хранение данных и коммуникация
- **Производительность**: Нативная скорость и оптимизации
- **Нативность**: Полная интеграция с операционной системой
- **Расширяемость**: Модульная архитектура с поддержкой плагинов

Проект готов к реализации дополнительных функций и масштабированию по мере роста требований пользователей.
