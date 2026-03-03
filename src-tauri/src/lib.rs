use keyring::Entry;
use serde_json::Value;
use std::collections::HashMap;
use tauri_plugin_store::StoreExt;

// Константа для идентификации сервиса в keyring
const KEYRING_SERVICE: &str = "chewback";
const KEYRING_USERNAME: &str = "refresh_token";

#[tauri::command]
async fn api_request(
    method: String,
    endpoint: String,
    body: Option<Value>,
    headers: Option<HashMap<String, String>>,
) -> Result<Value, String> {
    use once_cell::sync::Lazy;
    use reqwest::Client;

    // Конфигурация сервера
    #[cfg(debug_assertions)]
    const SERVER_URL: &str = "http://localhost:3000/api/v1/";

    #[cfg(not(debug_assertions))]
    const SERVER_URL: &str = "http://localhost:3000/api/v1/"; // Замените на продакшен URL

    // let client = Client::builder()
    //     .cookie_store(true)
    //     .build()
    //     .expect("Failed to create HTTP client");
    // Создаем статический клиент с поддержкой cookies
    static CLIENT: Lazy<Client> = Lazy::new(|| {
        Client::builder()
            .cookie_store(true) // Включаем поддержку cookies
            .build()
            .expect("Failed to create HTTP client")
    });
    let url = format!("{}{}", SERVER_URL, endpoint);

    // Создаем базовый запрос
    let mut request = match method.as_str() {
        "GET" => CLIENT.get(&url),
        "POST" => CLIENT.post(&url),
        "PUT" => CLIENT.put(&url),
        "DELETE" => CLIENT.delete(&url),
        "PATCH" => CLIENT.patch(&url),
        _ => return Err(format!("Unsupported method: {}", method)),
    };

    // Добавляем тело запроса если есть
    if let Some(body_data) = body {
        request = request.json(&body_data);
    }

    // Добавляем заголовки если есть
    let mut all_headers = headers.unwrap_or_default();

    // Добавляем стандартные заголовки если не указаны
    if !all_headers.contains_key("Content-Type") {
        all_headers.insert("Content-Type".to_string(), "application/json".to_string());
    }

    for (key, value) in all_headers {
        request = request.header(&key, value);
    }

    // Отправляем запрос
    let response = match request.send().await {
        Ok(resp) => resp,
        Err(e) => return Err(format!("Network error: {}", e)),
    };

    // Проверяем статус ответа
    let status = response.status();

    // Пытаемся получить текст ошибки если статус не успешный
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("HTTP {}: {}", status, error_text));
    }

    // Парсим JSON ответ
    match response.json::<Value>().await {
        Ok(json) => Ok(json),
        Err(e) => Err(format!("JSON parse error: {}", e)),
    }
}

#[tauri::command]
async fn save_refresh_token(_app_handle: tauri::AppHandle, token: String) -> Result<(), String> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)
        .map_err(|e| format!("Failed to create keyring entry: {e}"))?;

    entry
        .set_password(&token)
        .map_err(|e| format!("Failed to save refresh token to keyring: {e}"))?;
    Ok(())
}

// Команда для получения refresh token из Store
#[tauri::command]
async fn get_refresh_token(_app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)
        .map_err(|e| format!("Failed to create keyring entry: {e}"))?;

    match entry.get_password() {
        Ok(token) => Ok(Some(token)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("Failed to get refresh token from keyring: {e}")),
    }
}

// Команда для удаления refresh token из Store
#[tauri::command]
async fn clear_refresh_token(_app_handle: tauri::AppHandle) -> Result<(), String> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)
        .map_err(|e| format!("Failed to create keyring entry: {e}"))?;
    entry
        .delete_credential()
        .map_err(|e| format!("Failed to delete refresh token from keyring: {e}"))?;
    Ok(())
}

// Команда для сохранения данных пользователя в Store
#[tauri::command]
async fn save_user_data(app_handle: tauri::AppHandle, user_data: Value) -> Result<(), String> {
    let store = app_handle.store("store.json").map_err(|e| e.to_string())?;

    store.set("user_data".to_string(), user_data);
    store
        .save()
        .map_err(|e| format!("Failed to save store: {e}"))?;

    Ok(())
}

// Команда для получения данных пользователя из Store
#[tauri::command]
async fn get_user_data(app_handle: tauri::AppHandle) -> Result<Option<Value>, String> {
    let store = app_handle.store("store.json").map_err(|e| e.to_string())?;

    // Загружаем данные из файла
    store
        .reload()
        .map_err(|e| format!("Failed to load store: {e}"))?;

    match store.get("user_data") {
        Some(data) => Ok(Some(data.clone())),
        None => Ok(None),
    }
}

// Команда для очистки всех данных авторизации из Store
#[tauri::command]
async fn clear_auth_data(app_handle: tauri::AppHandle) -> Result<(), String> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)
        .map_err(|e| format!("Failed to create keyring entry: {e}"))?;

    let _ = entry.delete_credential();

    let store = app_handle.store("store.json").map_err(|e| e.to_string())?;
    store
        .reload()
        .map_err(|e| format!("Failed to load store: {e}"))?;

    // Удаляем все данные авторизации
    for (key, _) in store.entries() {
        store.delete(key);
    }
    
    store
        .save()
        .map_err(|e| format!("Failed to save store: {e}"))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let _ = app.store("store.json")?;

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
        .invoke_handler(tauri::generate_handler![
            api_request,
            save_refresh_token,
            get_refresh_token,
            clear_refresh_token,
            save_user_data,
            get_user_data,
            clear_auth_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
