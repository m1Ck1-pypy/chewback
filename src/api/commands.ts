import { invoke } from "@tauri-apps/api/core";
import type { User } from "./users";

// Сохранить refresh token в Store
export async function saveRefreshToken(token: string): Promise<void> {
  try {
    await invoke("save_refresh_token", { token });
    console.log("Refresh token saved to store");
  } catch (error) {
    console.error("Failed to save refresh token:", error);
    throw error;
  }
}

// Получить refresh token из Store
export async function getRefreshToken(): Promise<string | null> {
  try {
    const token = await invoke<string | null>("get_refresh_token");
    return token;
  } catch (error) {
    console.error("Failed to get refresh token:", error);
    return null;
  }
}

// Удалить refresh token из Store
export async function clearRefreshToken(): Promise<void> {
  try {
    await invoke("clear_refresh_token");
    console.log("Refresh token cleared from store");
  } catch (error) {
    console.error("Failed to clear refresh token:", error);
    throw error;
  }
}

// Сохранить данные пользователя в Store
export async function saveUserData(userData: User): Promise<void> {
  try {
    await invoke("save_user_data", { userData });
    console.log("User data saved to store");
  } catch (error) {
    console.error("Failed to save user data:", error);
    throw error;
  }
}

// Получить данные пользователя из Store
export async function getUserData(): Promise<User | null> {
  try {
    const userData = await invoke<User | null>("get_user_data");
    return userData;
  } catch (error) {
    console.error("Failed to get user data:", error);
    return null;
  }
}

// Очистить все данные авторизации из Store
export async function clearAuthData(): Promise<void> {
  try {
    await invoke("clear_auth_data");
    console.log("Auth data cleared from store");
  } catch (error) {
    console.error("Failed to clear auth data:", error);
    throw error;
  }
}

// Проверить, есть ли сохраненный refresh token
export async function hasStoredRefreshToken(): Promise<boolean> {
  const token = await getRefreshToken();
  return token !== null && token !== "";
}
