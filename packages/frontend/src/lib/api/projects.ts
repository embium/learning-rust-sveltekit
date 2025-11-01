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

export interface ApiError {
  error: string;
}

class ProjectsAPI {
  private async makeRequest(url: string, options: RequestInit = {}) {
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

  async createProject(project: CreateProject): Promise<void> {
    await this.makeRequest("/api/projects", {
      method: "POST",
      body: JSON.stringify(project),
    });
  }

  async listProjects(): Promise<Project[]> {
    const response = await this.makeRequest("/api/projects", {
      method: "GET",
    });
    return response || [];
  }

  async getProject(id: string): Promise<Project> {
    return await this.makeRequest(`/api/projects/${id}`, {
      method: "GET",
    });
  }

  async updateProject(id: string, update: UpdateProject): Promise<Project> {
    return await this.makeRequest(`/api/projects/${id}`, {
      method: "PUT",
      body: JSON.stringify(update),
    });
  }

  async deleteProject(id: string): Promise<void> {
    await this.makeRequest(`/api/projects/${id}`, {
      method: "DELETE",
    });
  }
}

export const projectsAPI = new ProjectsAPI();
