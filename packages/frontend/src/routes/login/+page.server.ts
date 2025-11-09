import type { PageServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ locals, url }) => {
  // If user is already authenticated, redirect to dashboard
  if (locals.user) {
    const redirectTo = url.searchParams.get('redirect') || '/dashboard';
    throw redirect(302, redirectTo);
  }

  return {};
};