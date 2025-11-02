import { makeRequest } from "./request";

export interface Account {
  name: string;
  email: string;
  password: string;
}

export interface AccountSettings {
  name: string | null;
  email: string;
}

export interface AccountUpdateRequest {
  name: string | null;
  email: string;
  current_password?: string;
  new_password?: string;
}

class AccountApi {
  async getAccountSettings(): Promise<AccountSettings> {
    const response = await makeRequest("/api/account", {
      method: "GET",
    });
    return response;
  }

  async updateAccountSettings(
    updateRequest: AccountUpdateRequest,
  ): Promise<void> {
    await makeRequest("/api/account", {
      method: "PUT",
      body: JSON.stringify(updateRequest),
    });
  }
}

export const accountAPI = new AccountApi();
