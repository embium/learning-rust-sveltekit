import type { PageServerLoad } from "./$types";
import { ProjectsAPI, type Project } from "$lib/api/projects";
import { redirect } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ locals, cookies }) => {
  const cookieHeader = cookies
    .getAll()
    .map((cookie) => `${cookie.name}=${cookie.value}`)
    .join("; ");

  if (!locals.user) {
    return redirect(302, "/login");
  }

  const projectsAPI = new ProjectsAPI(cookieHeader);
  const projects = await projectsAPI.listProjects();

  return {
    projects: projects.map((project: Project) => ({
      id: project.id,
      user_email: project.user_email,
      name: project.name,
      description: project.description,
      created_at: project.created_at,
      updated_at: project.updated_at,
    })),
  };
};
