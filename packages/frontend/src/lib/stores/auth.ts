import { browser } from '$app/environment';
import { writable, type Writable } from 'svelte/store';
import { authAPI, type User } from '$lib/api/auth';

type AuthState = {
  isAuthenticated: boolean;
  isLoading: boolean;
  user: User | null;
  lastCheckedAt?: number;
};

class AuthStore {
  private store: Writable<AuthState>;
  private monitorId: ReturnType<typeof setInterval> | null = null;
  subscribe: (run: (value: AuthState) => void) => () => void;

  constructor() {
    this.store = writable<AuthState>({
      isAuthenticated: false,
      isLoading: false,
      user: null,
    });
    this.subscribe = this.store.subscribe;
  }

  init = async () => {
    if (!browser) return;
    this.setLoading(true);
    try {
      const user = await authAPI.getCurrentUser();
      if (user) {
        this.setUser(user);
        // Start periodic monitor once authenticated
        this.startMonitor();
      } else {
        this.clearUser();
      }
    } catch (err) {
      // Attempt refresh if access expired
      try {
        await authAPI.refreshToken();
        const user = await authAPI.getCurrentUser();
        user ? this.setUser(user) : this.clearUser();
        if (user) this.startMonitor();
      } catch {
        this.clearUser();
      }
    } finally {
      this.setLoading(false);
    }
  };

  setUser = (user: User) => {
    this.store.update((s) => ({
      ...s,
      user,
      isAuthenticated: true,
      lastCheckedAt: Date.now(),
    }));
  };

  clearUser = () => {
    this.stopMonitor();
    this.store.set({ isAuthenticated: false, isLoading: false, user: null });
  };

  setLoading = (loading: boolean) => {
    this.store.update((s) => ({ ...s, isLoading: loading }));
  };

  checkSession = async () => {
    if (!browser) return;
    try {
      const user = await authAPI.getCurrentUser();
      if (user) {
        this.setUser(user);
        return true;
      }
      // If unauthorized, try refresh then fetch
      await authAPI.refreshToken();
      const refreshedUser = await authAPI.getCurrentUser();
      if (refreshedUser) {
        this.setUser(refreshedUser);
        return true;
      }
    } catch (err) {
      // swallow; fallthrough to clear
    }
    this.clearUser();
    return false;
  };

  startMonitor = (intervalMs: number = 5 * 60 * 1000) => {
    if (!browser) return;
    if (this.monitorId) return; // already running
    this.monitorId = setInterval(async () => {
      await this.checkSession();
    }, intervalMs);
  };

  stopMonitor = () => {
    if (this.monitorId) {
      clearInterval(this.monitorId);
      this.monitorId = null;
    }
  };
}

export const auth = new AuthStore();