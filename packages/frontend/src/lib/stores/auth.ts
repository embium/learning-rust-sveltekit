import { writable, derived } from "svelte/store";
import { browser } from "$app/environment";

export interface User {
  email: string;
}

export interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
}

const STORAGE_KEY_USER = "user_email";

// Initialize auth state from localStorage
function createAuthStore() {
  const initialState: AuthState = {
    user: null,
    isAuthenticated: false,
    isLoading: true,
  };

  // Load from localStorage if in browser
  if (browser) {
    const storedEmail = localStorage.getItem(STORAGE_KEY_USER);

    if (storedEmail) {
      try {
        initialState.user = { email: storedEmail };
        initialState.isAuthenticated = true;
      } catch (e) {
        console.error("Failed to parse stored auth data", e);
        localStorage.removeItem(STORAGE_KEY_USER);
      }
    }
    initialState.isLoading = false;
  }

  const { subscribe, set, update } = writable<AuthState>(initialState);

  return {
    subscribe,

    initializeAuth: async () => {
      if (!browser) return;

      // Get stored email
      const storedEmail = localStorage.getItem(STORAGE_KEY_USER);
      if (!storedEmail) {
        // No stored data, just set loading to false
        update((state) => ({ ...state, isLoading: false }));
        return;
      }

      try {
        // Dynamically import authAPI to avoid circular dependency
        const { authAPI } = await import("$lib/api/auth");
        const user = await authAPI.checkSession();

        if (!user || user.email !== storedEmail) {
          // Session invalid or email mismatch, clear auth
          console.log("❌ Session validation failed, clearing auth");
          set({
            user: null,
            isAuthenticated: false,
            isLoading: false,
          });
          localStorage.removeItem(STORAGE_KEY_USER);
        } else {
          // Session valid, ensure state is correct
          console.log("✅ Session validated successfully");
          update((state) => ({
            ...state,
            user: { email: user.email },
            isAuthenticated: true,
            isLoading: false,
          }));
        }
      } catch (error) {
        console.error("❌ Failed to validate session:", error);
        // On error, clear auth state
        set({
          user: null,
          isAuthenticated: false,
          isLoading: false,
        });
        localStorage.removeItem(STORAGE_KEY_USER);
      }
    },

    setUser: (user: User) => {
      console.log("👤 Setting user in auth store:", user.email);

      const newState: AuthState = {
        user,
        isAuthenticated: true,
        isLoading: false,
      };

      set(newState);

      // Persist to localStorage
      if (browser) {
        localStorage.setItem(STORAGE_KEY_USER, user.email);
      }
    },

    clearUser: () => {
      console.log("🚪 Clearing user from auth store");

      set({
        user: null,
        isAuthenticated: false,
        isLoading: false,
      });

      // Clear localStorage
      if (browser) {
        localStorage.removeItem(STORAGE_KEY_USER);
      }
    },

    setLoading: (isLoading: boolean) => {
      update((state) => ({ ...state, isLoading }));
    },

    checkAuthStatus: async () => {
      if (!browser) return;

      console.log("🔍 Starting auth status check...");

      // Don't set loading here - let the component handle loading states
      try {
        const { authAPI } = await import("$lib/api/auth");
        const user = await authAPI.checkSession();

        if (user) {
          console.log(
            "✅ Auth check successful, user authenticated:",
            user.email,
          );

          const newState: AuthState = {
            user,
            isAuthenticated: true,
            isLoading: false,
          };

          set(newState);

          // Update localStorage
          localStorage.setItem(STORAGE_KEY_USER, user.email);
        } else {
          console.log("❌ Auth check failed, no valid session");

          set({
            user: null,
            isAuthenticated: false,
            isLoading: false,
          });

          localStorage.removeItem(STORAGE_KEY_USER);
        }
      } catch (error) {
        console.error("❌ Auth check failed with error:", error);

        set({
          user: null,
          isAuthenticated: false,
          isLoading: false,
        });

        localStorage.removeItem(STORAGE_KEY_USER);
      }
    },
  };
}

export const auth = createAuthStore();

// Derived stores for easy access
export const currentUser = derived(auth, ($auth) => $auth.user);
export const isAuthenticated = derived(auth, ($auth) => $auth.isAuthenticated);
export const isLoading = derived(auth, ($auth) => $auth.isLoading);
