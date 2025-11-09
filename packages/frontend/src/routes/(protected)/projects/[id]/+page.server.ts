import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { ProjectsAPI } from '$lib/api/projects';

export const load: PageServerLoad = async ({ locals, params, cookies }) => {
  try {
    const cookieHeader = cookies
      .getAll()
      .map((cookie) => `${cookie.name}=${cookie.value}`)
      .join('; ');

    if (!locals.user) {
      return redirect(302, '/login');
    }

    const projectId = params.id;
    const projectsAPI = new ProjectsAPI(cookieHeader);
    const project = await projectsAPI.getProject(projectId);

    return {
      project: {
        id: project.id,
        user_email: project.user_email,
        name: project.name,
        description: project.description,
        created_at: project.created_at,
        updated_at: project.updated_at,
      },
    };
  } catch (err) {
    console.error('Error loading project:', err);

    // If it's a SvelteKit error, re-throw it
    if (err && typeof err === 'object' && 'status' in err) {
      throw err;
    }

    // Otherwise, throw a generic error
    throw error(500, 'Failed to load project');
  }
};
