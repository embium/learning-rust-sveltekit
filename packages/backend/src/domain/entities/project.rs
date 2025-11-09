use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub user_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectWithOwnerEmail {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub user_email: Option<String>,
}

impl Project {
    pub fn new(user_id: Option<String>, name: String, description: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            name,
            description,
            created_at: chrono::Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn update(&mut self, name: String, description: Option<String>) {
        self.name = name;
        self.description = description;
        self.updated_at = Some(chrono::Utc::now());
    }

    pub fn delete(&mut self) {
        self.deleted_at = Some(chrono::Utc::now());
    }
}
