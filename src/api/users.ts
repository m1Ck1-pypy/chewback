import type { components } from "../bindings/types";
import BaseApiClient from "./base-client";

// type CreateUserRequest = components["schemas"]["CreateUserRequest"];
// export type UpdateUserRequest = components["schemas"]["UpdateUserRequest"];
// export type UserResponse = components["schemas"]["UserResponse"];
export type MessageResponse = components["schemas"]["MessageResponse"];
export type User = components["schemas"]["AuthResponseWithTokens"]["user"];

// Подмодуль для Auth
class UsersApi extends BaseApiClient {
  getUsers(): Promise<User[]> {
    return this.get<User[]>("users");
  }
  getUserById(id: string): Promise<User> {
    return this.get<User>(`users/${id}`);
  }
  // updateUser(id: string, user: UpdateUserRequest): Promise<UserResponse> {
  //   return this.put<UserResponse, UpdateUserRequest>(`users/${id}`, user);
  // }
  deleteUser(id: string): Promise<MessageResponse> {
    return this.delete<MessageResponse>(`users/${id}`);
  }
}

export default UsersApi;
