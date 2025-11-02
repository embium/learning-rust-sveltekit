import { auth } from "$lib/stores/auth";
import { makeRequest } from "./request";

export interface LoginRequest {
  email: string;
  password: string;
}

export interface SignupRequest {
  email: string;
  password: string;
}

class AuthAPI {
  async login(credentials: LoginRequest): Promise<void> {
    auth.setLoading(true);

    try {
      const response = await makeRequest("/api/login", {
        method: "POST",
        body: JSON.stringify(credentials),
      });

      // Store email in localStorage for session persistence
      localStorage.setItem("user_email", credentials.email);

      // Update auth store
      auth.setUser({ email: credentials.email });
    } catch (error) {
      auth.setLoading(false);
      throw error;
    }
  }

  async signup(userData: SignupRequest): Promise<void> {
    await makeRequest("/api/signup", {
      method: "POST",
      body: JSON.stringify(userData),
    });
  }

  async logout(): Promise<void> {
    try {
      await makeRequest("/api/logout", {
        method: "POST",
      });
    } catch (error) {
      // Even if logout fails on server, clear local state
      console.error("Logout error:", error);
    } finally {
      localStorage.removeItem("user_email");
      auth.clearUser();
    }
  }

  async checkSession(): Promise<{ email: string } | null> {
    try {
      const response = await makeRequest("/api/me", {
        method: "GET",
      });

      if (response && response.authenticated) {
        return { email: response.email };
      }
      return null;
      // eslint-disable-next-line @typescript-eslint/no-unused-vars
    } catch (error) {
      return null;
    }
  }
}

export const authAPI = new AuthAPI();
