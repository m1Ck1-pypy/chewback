import BaseApiClient from "./base-client";
import { components } from "../bindings/types";
import { User } from "./users";
import {
  clearAuthData,
  getRefreshToken,
  getUserData,
  saveRefreshToken,
  saveUserData,
} from "./commands";

export type CreateUserRequest = components["schemas"]["CreateUserRequest"];
export type AuthResponseWithTokens =
  components["schemas"]["AuthResponseWithTokens"];
export type LoginRequest = components["schemas"]["LoginRequest"];
export type RefreshTokenRequest = components["schemas"]["RefreshTokenRequest"];
export type LogoutResponse = components["schemas"]["MessageResponse"];

class AuthApi extends BaseApiClient {
  constructor() {
    super();
  }

  endpoint(str: string): string {
    return `auth/${str}`;
  }

  async register(data: CreateUserRequest): Promise<AuthResponseWithTokens> {
    const response = await this.post<AuthResponseWithTokens, CreateUserRequest>(
      this.endpoint("register"),
      data,
    );

    this.storeAuthData(response);

    return response;
  }

  async login(data: LoginRequest): Promise<AuthResponseWithTokens> {
    const response = await this.post<AuthResponseWithTokens, LoginRequest>(
      this.endpoint("login"),
      data,
    );

    this.storeAuthData(response);

    return response;
  }

  async refreshToken(
    data?: RefreshTokenRequest,
  ): Promise<AuthResponseWithTokens> {
    // Если refresh token не передан, пытаемся получить из Store
    if (!data?.refresh_token) {
      const storedToken = await getRefreshToken();
      if (!storedToken) {
        throw new Error("No refresh token found in storage");
      }
      data = { refresh_token: storedToken };
    }

    const response = await this.post<
      AuthResponseWithTokens,
      RefreshTokenRequest
    >(this.endpoint("refresh"), data);

    // Сохраняем новый refresh token и данные пользователя
    this.storeAuthData(response);

    return response;
  }

  async logout(): Promise<LogoutResponse> {
    // Очищаем все данные авторизации из Store и keyring-данные (refresh_token)
    await clearAuthData();
    return this.post<LogoutResponse>(this.endpoint("logout"));
  }

  getMe(): Promise<User> {
    return this.get<User>(this.endpoint("me"));
  }

  private async storeAuthData(response: AuthResponseWithTokens): Promise<void> {
    if (response.refresh_token) {
      await saveRefreshToken(response.refresh_token);
    }
    await saveUserData(response.user);
  }

  async getStoredRefreshToken(): Promise<string | null> {
    return getRefreshToken();
  }

  async getStoredUserData(): Promise<User | null> {
    return getUserData();
  }

  async hasStoredAuthData(): Promise<boolean> {
    const token = await getRefreshToken();
    const userData = await getUserData();
    return token !== null && token !== "" && userData !== null;
  }
}

export default AuthApi;
