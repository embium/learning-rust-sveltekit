import { PUBLIC_API_URL } from "$env/static/public";

export interface ApiError {
  error: string;
}

function getCsrfToken(): string | null {
  if (typeof document === 'undefined') return null;
  const match = document.cookie.match(/(?:^|; )csrf_token=([^;]*)/);
  return match ? decodeURIComponent(match[1]) : null;
}

export async function makeRequest(endpoint: string, options: RequestInit = {}) {
  const url = `${PUBLIC_API_URL}${endpoint}`;
  const method = (options.method || 'GET').toString().toUpperCase();
  const csrf = getCsrfToken();
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    ...(options.headers as Record<string, string>),
  };
  if (csrf && method !== 'GET' && method !== 'HEAD' && !headers['X-CSRF-Token']) {
    headers['X-CSRF-Token'] = csrf;
  }

  const response = await fetch(url, {
    ...options,
    credentials: "include",
    headers,
  });

  // Check if response has content
  const contentType = response.headers.get("content-type");
  let responseData = null;

  if (contentType && contentType.includes("application/json")) {
    responseData = await response.json();
  }

  if (!response.ok) {
    // Handle different error scenarios
    if (responseData && responseData.error) {
      throw new Error(responseData.error);
    } else if (response.status === 409) {
      // Handle conflict errors (like account already exists)
      const errorMessage =
        responseData?.error ||
        "An account with this email already exists. Please log in with your email and password instead.";
      throw new Error(errorMessage);
    } else if (response.status === 401) {
      const errorMessage = responseData?.message || "Unauthorized";
      throw new Error(errorMessage);
    } else {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }
  }

  return responseData;
}
