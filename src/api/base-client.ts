import { invoke } from "@tauri-apps/api/core";

export enum HttpMethod {
  GET = "GET",
  POST = "POST",
  PUT = "PUT",
  DELETE = "DELETE",
  PATCH = "PATCH",
}

export interface ApiRequestOptions {
  method: HttpMethod;
  endpoint: string;
  data?: unknown;
  headers?: Record<string, string>;
  params?: Record<string, string | number | boolean>;
}

export interface ApiResponse<T = unknown> {
  data: T;
  status: number;
  headers?: Record<string, string>;
}

export class ApiError extends Error {
  constructor(
    message: string,
    public status?: number,
    public data?: unknown,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

// Базовый класс API клиента для Tauri
export class BaseApiClient {
  // Универсальный метод для отправки запросов через Tauri
  protected async request<T = unknown>(options: ApiRequestOptions): Promise<T> {
    const { method, endpoint, data, headers = {}, params = {} } = options;

    try {
      // Добавляем query параметры если есть
      let fullEndpoint = endpoint;
      if (Object.keys(params).length > 0) {
        const queryParams = new URLSearchParams();
        Object.entries(params).forEach(([key, value]) => {
          queryParams.append(key, String(value));
        });
        fullEndpoint += `?${queryParams.toString()}`;
      }

      // Вызываем Tauri команду для отправки запроса
      const result = await invoke<unknown>("api_request", {
        method,
        endpoint: fullEndpoint,
        body: data,
        headers,
      });

      return result as T;
    } catch (error) {
      // Обработка ошибок от Tauri
      if (typeof error === "string") {
        // Пытаемся распарсить ошибку в формате "HTTP {status}: {message}"
        const match = error.match(/^HTTP (\d+):\s*(.*)$/);
        if (match) {
          const [, status, message] = match;
          throw new ApiError(message, parseInt(status, 10), error);
        }
        throw new ApiError(error);
      }

      if (error instanceof Error) {
        throw new ApiError(error.message);
      }

      throw new ApiError("Unknown API error");
    }
  }

  // Конкретные методы для каждого HTTP метода
  protected get<T = unknown>(
    endpoint: string,
    params?: Record<string, string | number | boolean>,
    headers?: Record<string, string>,
  ): Promise<T> {
    return this.request<T>({
      method: HttpMethod.GET,
      endpoint,
      params,
      headers,
    });
  }

  protected post<T = unknown, D = unknown>(
    endpoint: string,
    data?: D,
    headers?: Record<string, string>,
  ): Promise<T> {
    return this.request<T>({
      method: HttpMethod.POST,
      endpoint,
      data,
      headers,
    });
  }

  protected put<T = unknown, D = unknown>(
    endpoint: string,
    data?: D,
    headers?: Record<string, string>,
  ): Promise<T> {
    return this.request<T>({
      method: HttpMethod.PUT,
      endpoint,
      data,
      headers,
    });
  }

  protected patch<T = unknown, D = unknown>(
    endpoint: string,
    data?: D,
    headers?: Record<string, string>,
  ): Promise<T> {
    return this.request<T>({
      method: HttpMethod.PATCH,
      endpoint,
      data,
      headers,
    });
  }

  protected delete<T = unknown>(
    endpoint: string,
    headers?: Record<string, string>,
  ): Promise<T> {
    return this.request<T>({
      method: HttpMethod.DELETE,
      endpoint,
      headers,
    });
  }

  // // Вспомогательный метод для загрузки файлов
  // protected async uploadFile<T = unknown>(
  //   endpoint: string,
  //   file: File,
  //   additionalData?: Record<string, unknown>,
  //   headers?: Record<string, string>,
  // ): Promise<T> {
  //   // Для загрузки файлов нужно будет создать отдельную Tauri команду
  //   // или использовать FormData через специальную команду
  //   throw new ApiError("File upload not implemented yet");
  // }

  // // Вспомогательный метод для скачивания файлов
  // protected async downloadFile(
  //   endpoint: string,
  //   params?: Record<string, string | number | boolean>,
  //   headers?: Record<string, string>,
  // ): Promise<Blob> {
  //   // Для скачивания файлов нужна отдельная реализация
  //   throw new ApiError("File download not implemented yet");
  // }
}

export default BaseApiClient;
