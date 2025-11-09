import type { LayoutServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';
import { AuthAPI } from '$lib/api/auth';

export const load: LayoutServerLoad = async ({ cookies, url }) => {
  const cookieHeader = cookies
    .getAll()
    .map((cookie) => `${cookie.name}=${cookie.value}`)
    .join('; ');

  const auth = new AuthAPI(cookieHeader);
  const user = await auth.checkSession();

  if (!user) {
    const loginUrl = `/login?redirect=${encodeURIComponent(url.pathname)}`;
    throw redirect(302, loginUrl);
  }

  return { user };
};