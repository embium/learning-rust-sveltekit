import type { Handle, HandleFetch } from '@sveltejs/kit';
import { AuthAPI } from '$lib/api/auth';

function buildCookieHeader(cookies: import('@sveltejs/kit').Cookies) {
  return cookies
    .getAll()
    .map((c) => `${c.name}=${c.value}`)
    .join('; ');
}

export const handle: Handle = async ({ event, resolve }) => {
  // Ensure CSRF token cookie exists (double-submit token pattern)
  const existingCsrf = event.cookies.get('csrf_token');
  if (!existingCsrf) {
    const token = crypto.randomUUID();
    const isSecure = process.env.NODE_ENV !== 'development';
    event.cookies.set('csrf_token', token, {
      path: '/',
      httpOnly: false,
      sameSite: 'strict',
      secure: isSecure,
      maxAge: 60 * 60 * 24 * 7, // 7 days
    });
  }

  // Validate session and attach current user to locals
  try {
    const cookieHeader = buildCookieHeader(event.cookies);
    if (cookieHeader.includes('access_token=')) {
      const api = new AuthAPI(cookieHeader);
      const user = await api.checkSession();
      if (user) {
        event.locals.user = user;
      }
    }
  } catch (err) {
    // ignore errors; user remains unauthenticated in locals
  }

  return resolve(event);
};

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
  // Forward cookies for server-side requests if missing
  const cookieHeader = buildCookieHeader(event.cookies);
  const req = new Request(request, {
    headers: new Headers(request.headers),
  });

  // Attach Cookie header for same-origin or our API domain
  const url = new URL(req.url);
  const apiBase = process.env.PUBLIC_API_URL || '';
  const isApiRequest = apiBase && req.url.startsWith(apiBase);
  const isSameOrigin = url.origin === event.url.origin;

  if ((isApiRequest || isSameOrigin) && !req.headers.has('Cookie') && cookieHeader) {
    req.headers.set('Cookie', cookieHeader);
  }

  // Add CSRF header for state-changing requests
  const method = req.method.toUpperCase();
  if (method !== 'GET' && method !== 'HEAD') {
    const csrf = event.cookies.get('csrf_token');
    if (csrf && !req.headers.has('X-CSRF-Token')) {
      req.headers.set('X-CSRF-Token', csrf);
    }
  }

  return fetch(req);
};