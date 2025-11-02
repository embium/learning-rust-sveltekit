export interface ApiError {
  error: string;
}

export async function makeRequest(url: string, options: RequestInit = {}) {
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
