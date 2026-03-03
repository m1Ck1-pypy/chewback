import {
  createContext,
  createSignal,
  createEffect,
  type JSX,
  onCleanup,
} from "solid-js";
import { api } from "../api";
import type { User } from "../api/users";

interface AuthContextType {
  user: () => User | null;
  isLoading: () => boolean;
  checkAuth: () => Promise<void>;
  logout: () => Promise<void>;
  isAuthenticated: () => boolean;
  refreshToken: () => Promise<boolean>;
  restoreSession: () => Promise<boolean>;
}

const AuthContext = createContext<AuthContextType>();
export { AuthContext };

// Интервал для проверки и обновления токена (каждые 30 минут)
const TOKEN_CHECK_INTERVAL = 30 * 60 * 1000; // 30 минут в миллисекундах

export function AuthProvider(props: { children: JSX.Element }) {
  const [user, setUser] = createSignal<User | null>(null);
  const [isLoading, setIsLoading] = createSignal(true);
  let refreshInterval: number | null = null;

  // Функция для восстановления сессии из Store
  const restoreSession = async (): Promise<boolean> => {
    try {
      // Проверяем, есть ли сохраненные данные авторизации
      const hasAuthData = await api.auth.hasStoredAuthData();

      if (!hasAuthData) {
        return false;
      }

      // Пытаемся получить сохраненные данные пользователя для мгновенного отображения
      const storedUser = await api.auth.getStoredUserData();
      if (storedUser) {
        setUser(storedUser);
      }

      // Пытаемся обновить токен с помощью сохраненного refresh token
      const refreshed = await refreshToken();
      return refreshed ? true : false;
    } catch (error) {
      console.error("Error restoring session:", error);
      return false;
    }
  };

  // Функция для обновления токена через refresh token
  const refreshToken = async (): Promise<boolean> => {
    try {
      // Пытаемся обновить токен
      const response = await api.auth.refreshToken();

      // Обновляем данные пользователя
      setUser(response.user);
      return true;
    } catch (error) {
      console.error("Refresh token failed:", error);
      return false;
    }
  };

  const checkAuth = async () => {
    setIsLoading(true);
    try {
      const currentUser = await api.auth.getMe();
      setUser(currentUser);
    } catch (error) {
      // Если не удалось получить данные пользователя, пытаемся восстановить сессию
      const restored = await restoreSession();
      if (!restored) {
        console.log("Failed to restore session, user not authenticated");
        setUser(null);
      }
    } finally {
      setIsLoading(false);
    }
  };

  const logout = async () => {
    try {
      await api.auth.logout();
    } finally {
      setUser(null);
      if (refreshInterval) {
        clearInterval(refreshInterval);
        refreshInterval = null;
      }
    }
  };

  // Запускаем периодическую проверку и обновление токена
  const startTokenRefreshInterval = () => {
    // Останавливаем предыдущий интервал если есть
    if (refreshInterval) {
      clearInterval(refreshInterval);
    }

    // Запускаем новый интервал
    refreshInterval = setInterval(async () => {
      if (user()) {
        try {
          await refreshToken();
        } catch (error) {
          console.error("Periodic token refresh failed:", error);
        }
      }
    }, TOKEN_CHECK_INTERVAL) as unknown as number;
  };

  // Проверяем авторизацию только один раз при монтировании провайдера
  createEffect(() => {
    checkAuth().then(() => {
      if (user()) {
        startTokenRefreshInterval();
      }
    });
  });

  // Очищаем интервал при размонтировании
  onCleanup(() => {
    if (refreshInterval) {
      clearInterval(refreshInterval);
    }
  });

  const value: AuthContextType = {
    user,
    isLoading,
    checkAuth,
    logout,
    isAuthenticated: () => !!user(),
    refreshToken,
    restoreSession,
  };

  return (
    <AuthContext.Provider value={value}>{props.children}</AuthContext.Provider>
  );
}
