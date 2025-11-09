use serde::Deserialize;
use validator::Validate;

use crate::domain::entities::project::Project;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateOrUpdateProject {
    #[validate(length(min = 1, max = 255, message = "Name is required"))]
    pub name: String,

    pub description: Option<String>,
}

impl From<&CreateOrUpdateProject> for Project {
    fn from(req: &CreateOrUpdateProject) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: None,
            name: req.name.clone(),
            description: req.description.clone(),
            created_at: chrono::Utc::now(),
            updated_at: Some(chrono::Utc::now()),
            deleted_at: None,
        }
    }
}
