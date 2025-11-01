import { auth } from "$lib/stores/auth";

export interface LoginRequest {
  email: string;
  password: string;
}

export interface SignupRequest {
  email: string;
  password: string;
}

export interface ApiError {
  error: string;
}

class AuthAPI {
  private async makeRequest(url: string, options: RequestInit = {}) {
    const response = await fetch(url, {
      ...options,
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
        ...options.headers,
      },
    });

    if (!response.ok) {
      const errorData: ApiError = await response.json().catch(() => ({
        error: `HTTP ${response.status}: ${response.statusText}`,
      }));
      throw new Error(errorData.error);
    }

    // Check if response has content
    const contentType = response.headers.get("content-type");
    if (contentType && contentType.includes("application/json")) {
      return await response.json();
    }

    return null;
  }

  async login(credentials: LoginRequest): Promise<void> {
    auth.setLoading(true);

    try {
      console.log("🔐 Starting login request...");
      const response = await this.makeRequest("/api/login", {
        method: "POST",
        body: JSON.stringify(credentials),
      });

      console.log("✅ Login response:", response);

      // Store email in localStorage for session persistence
      localStorage.setItem("user_email", credentials.email);

      // Update auth store
      auth.setUser({ email: credentials.email });
      console.log("✅ User authenticated and stored in state");
    } catch (error) {
      console.error("❌ Login failed:", error);
      auth.setLoading(false);
      throw error;
    }
  }

  async signup(userData: SignupRequest): Promise<void> {
    await this.makeRequest("/api/signup", {
      method: "POST",
      body: JSON.stringify(userData),
    });
  }

  async logout(): Promise<void> {
    try {
      await this.makeRequest("/api/logout", {
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
      console.log("🔍 Checking session status...");
      const response = await this.makeRequest("/api/me", {
        method: "GET",
      });

      console.log("📋 Session check response:", response);

      if (response && response.authenticated) {
        console.log("✅ Session valid for user:", response.email);
        return { email: response.email };
      }
      console.log("❌ No valid session found");
      return null;
    } catch (error) {
      console.error("❌ Session check failed:", error);
      return null;
    }
  }
}

export const authAPI = new AuthAPI();
