import { makeRequest } from "./request";

export interface User {
  name: string;
  email: string;
  password: string;
}

export interface UserSettings {
  fullname: string | null;
  email: string;
  provider: string;
}

export interface UserUpdateRequest {
  name: string | null;
  email: string;
  current_password?: string;
  new_password?: string;
}

class UserApi {
  async getUserSettings(): Promise<UserSettings> {
    const response = await makeRequest("/api/v1/user/settings", {
      method: "GET",
    });
    return response.data;
  }

  async updateUserSettings(updateRequest: UserUpdateRequest): Promise<void> {
    await makeRequest("/api/v1/user/settings", {
      method: "PUT",
      body: JSON.stringify(updateRequest),
    });
  }
}

export const userAPI = new UserApi();
