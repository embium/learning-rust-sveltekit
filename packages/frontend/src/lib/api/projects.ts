import { makeRequest } from "./request";

export interface Project {
  id?: string;
  user_email: string;
  name: string;
  description?: string;
  created_at?: string;
  last_updated?: string;
}

export interface CreateProject {
  user_email: string;
  name: string;
  description?: string;
}

export interface UpdateProject {
  name?: string;
  description?: string;
  update_timestamp: number;
}

class ProjectsAPI {
  async createProject(project: CreateProject): Promise<void> {
    await makeRequest("/api/projects", {
      method: "POST",
      body: JSON.stringify(project),
    });
  }

  async listProjects(): Promise<Project[]> {
    const response = await makeRequest("/api/projects", {
      method: "GET",
    });
    return response || [];
  }

  async getProject(id: string): Promise<Project> {
    return await makeRequest(`/api/projects/${id}`, {
      method: "GET",
    });
  }

  async updateProject(id: string, update: UpdateProject): Promise<Project> {
    return await makeRequest(`/api/projects/${id}`, {
      method: "PUT",
      body: JSON.stringify(update),
    });
  }

  async deleteProject(id: string): Promise<void> {
    await makeRequest(`/api/projects/${id}`, {
      method: "DELETE",
    });
  }
}

export const projectsAPI = new ProjectsAPI();
