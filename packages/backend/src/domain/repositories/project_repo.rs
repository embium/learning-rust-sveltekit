use crate::{
    domain::entities::project::{Project, ProjectWithOwnerEmail},
    infra::errors::app_error::AppError,
};

#[async_trait::async_trait]
pub trait ProjectRepository {
    async fn find_all_by_user_id(&self, user_id: &str) -> Result<Vec<Project>, AppError>;
    async fn find_by_id_and_user_id(&self, id: &str, user_id: &str) -> Result<ProjectWithOwnerEmail, AppError>;
    async fn create(&self, entity: Project) -> Result<Project, AppError>;
    async fn update(&self, id: &str, user_id: &str, entity: Project) -> Result<ProjectWithOwnerEmail, AppError>;
    async fn delete(&self, id: &str, user_id: &str) -> Result<(), AppError>;
}
