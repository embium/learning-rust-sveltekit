import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals }) => {
  // Pass the authenticated user from server hooks to the client
  return {
    user: locals.user || null,
  };
};