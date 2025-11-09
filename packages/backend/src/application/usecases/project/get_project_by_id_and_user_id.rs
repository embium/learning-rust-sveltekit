use std::sync::Arc;

use crate::{
    domain::{entities::project::ProjectWithOwnerEmail, repositories::project_repo::ProjectRepository},
    infra::errors::app_error::AppError,
};

#[derive(Clone)]
pub struct GetProjectByIdAndUserId<R> {
    project_repo: Arc<R>,
}

impl<R> GetProjectByIdAndUserId<R>
where
    R: ProjectRepository,
{
    pub fn new(project_repo: Arc<R>) -> Self {
        Self { project_repo }
    }

    pub async fn execute(&self, project_id: &str, user_id: &str) -> Result<ProjectWithOwnerEmail, AppError> {
        let project = self.project_repo.find_by_id_and_user_id(project_id, user_id).await?;

        Ok(project)
    }
}
