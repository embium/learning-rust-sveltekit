use std::sync::Arc;

use crate::{
    application::dto::project::create_update_project_request::CreateOrUpdateProject,
    domain::{entities::project::{Project, ProjectWithOwnerEmail}, repositories::project_repo::ProjectRepository},
    infra::errors::app_error::AppError,
};

#[derive(Clone)]
pub struct UpdateProject<R> {
    project_repo: Arc<R>
}

impl<R> UpdateProject<R>
where
    R: ProjectRepository,
{
    pub fn new(project_repo: Arc<R>) -> Self {
        Self { project_repo }
    }

    pub async fn execute(&self, id: &str, user_id: &str, req: CreateOrUpdateProject) -> Result<ProjectWithOwnerEmail, AppError> {

        let mut project_req = Project::from(&req);

        project_req.user_id = Some(user_id.to_string());

        let project = self.project_repo.update(id, user_id, project_req).await?;

        Ok(project)
    }
}
