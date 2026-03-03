import { BaseApiClient, HttpMethod, ApiError } from "./base-client";
import AuthApi from "./auth";
import UsersApi from "./users";

// Главный API клиент, расширяющий базовый функционал
export class ApiClient extends BaseApiClient {
  readonly auth: AuthApi;
  readonly users: UsersApi;

  constructor() {
    super();
    this.auth = new AuthApi();
    this.users = new UsersApi();
  }

  // Публичные методы для удобного использования
  // Можно добавить часто используемые методы здесь

  /**
   * Проверка здоровья сервера
   */
  async healthCheck(): Promise<{ status: string }> {
    return this.get("health");
  }

  /**
   * Универсальный метод запроса с явным указанием типа
   */
  // async fetch<T = unknown>(
  //   method: HttpMethod,
  //   endpoint: string,
  //   data?: unknown,
  //   headers?: Record<string, string>,
  // ): Promise<T> {
  //   switch (method) {
  //     case HttpMethod.GET:
  //       return this.get(endpoint, undefined, headers);
  //     case HttpMethod.POST:
  //       return this.post(endpoint, data, headers);
  //     case HttpMethod.PUT:
  //       return this.put(endpoint, data, headers);
  //     case HttpMethod.PATCH:
  //       return this.patch(endpoint, data, headers);
  //     case HttpMethod.DELETE:
  //       return this.delete(endpoint, headers);
  //     default:
  //       throw new ApiError(`Unsupported HTTP method: ${method}`);
  //   }
  // }
}

// Экспортируем инстанс для использования во всем приложении
export const api = new ApiClient();

// Экспортируем типы для удобного импорта
export type { ApiError };
export { HttpMethod };
