import { makeRequest } from './request';

export interface LoginRequest {
  email: string;
  password: string;
}

export interface SignupRequest {
  email: string;
  password: string;
}

export interface User {
  id: string;
  email: string;
  fullname?: string;
  avatar_url?: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface AuthResponse {
  user: User;
}

export class AuthAPI {
  cookieHeader?: string;

  constructor(cookieHeader?: string) {
    this.cookieHeader = cookieHeader;
  }

  async login(credentials: LoginRequest): Promise<User | null> {
    const response = await makeRequest('/oauth/email/login', {
      method: 'POST',
      body: JSON.stringify(credentials),
      headers: {
        Cookie: this.cookieHeader || '',
      },
    });

    if (response?.success === false) {
      throw new Error(response?.message || 'Login failed');
    }
    return await this.ensureSession();
  }

  async signup(userData: SignupRequest): Promise<void> {
    const response = await makeRequest('/oauth/email/register', {
      method: 'POST',
      body: JSON.stringify(userData),
      headers: {
        Cookie: this.cookieHeader || '',
      },
    });

    if (response?.success === false) {
      throw new Error(response?.message || 'Signup failed');
    }
  }

  async logout(): Promise<void> {
    await makeRequest('/api/v1/auth/logout', {
      method: 'DELETE',
      headers: {
        Cookie: this.cookieHeader || '',
      },
    });
  }

  async getCurrentUser(): Promise<User | null> {
    try {
      const response = await makeRequest('/api/v1/auth/current-user', {
        method: 'GET',
        headers: {
          Cookie: this.cookieHeader || '',
        },
      });

      if (response && response.data) {
        const user = response.data as User;
        return user;
      }
      return null;
    } catch (error) {
      console.error('Failed to get current user:', error);
      return null;
    }
  }

  async checkSession(): Promise<User | null> {
    return await this.getCurrentUser();
  }

  // OAuth functionality
  async getGoogleAuthUrl(): Promise<string> {
    const response = await makeRequest('/oauth/google/get-url', {
      method: 'GET',
    });
    return response?.data || '';
  }

  async handleGoogleCallback(code: string): Promise<User | null> {
    await makeRequest(
      `/oauth/google/callback?code=${encodeURIComponent(code)}`,
      {
        method: 'GET',
      }
    );

    // Ensure user is available after successful OAuth
    return await this.ensureSession();
  }

  async refreshToken(): Promise<void> {
    try {
      await makeRequest('/oauth/refresh-token', {
        method: 'GET',
        headers: {
          Cookie: this.cookieHeader || '',
        },
      });
    } catch (error) {
      console.error('Token refresh failed:', error);
      throw error;
    }
  }

  async ensureSession(): Promise<User | null> {
    const user = await this.getCurrentUser();
    if (user) return user;
    // Try refresh then fetch
    try {
      await this.refreshToken();
      return await this.getCurrentUser();
    } catch {
      return null;
    }
  }
}

export const authAPI = new AuthAPI();
