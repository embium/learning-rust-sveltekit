use sqlx::postgres::{PgPool, PgPoolOptions};
use serde::{Serialize, Deserialize, Serializer};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Error
    },
    Argon2
};
use thiserror::Error;
use sqlx::Row;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;
use validator::Validate;
use time::format_description;

fn serialize_primitive_datetime_option<S>(
    dt: &Option<sqlx::types::time::PrimitiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match dt {
        Some(datetime) => {
            let offset_datetime = datetime.assume_utc();
            serializer.serialize_str(&offset_datetime.format(&format_description::well_known::Rfc3339).map_err(serde::ser::Error::custom)?)
        }
        None => serializer.serialize_none(),
    }
}

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountSettings {
    pub name: Option<String>,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountUpdateRequest {
    pub name: Option<String>,
    pub email: String,
    pub current_password: Option<String>,
    pub new_password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Project {
    pub id: Option<uuid::Uuid>,

    #[validate(email)]
    pub user_email: String,

    #[validate(length(min = 1, message = "Project must have a name"))]
    pub name: String,

    pub description: Option<String>,

    #[serde(serialize_with = "serialize_primitive_datetime_option")]
    pub created_at: Option<sqlx::types::time::PrimitiveDateTime>,

    #[serde(serialize_with = "serialize_primitive_datetime_option")]
    pub last_updated: Option<sqlx::types::time::PrimitiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub description: Option<String>,
    pub update_timestamp: i64,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
            {
                Ok(pool) => pool,
                Err(e) => panic!("Couldn't establish database connection: {}", e),
            };
        Store {
            connection: db_pool,
        }
    }

    pub async fn authenticate_user(&self, email: &str, password: &str) -> Result<bool, StoreError> {
        let row = sqlx::query("SELECT password FROM accounts WHERE email = $1")
            .bind(email)
            .fetch_one(&self.connection)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => StoreError::UserNotFound,
                other => StoreError::SqlxError(other),
            })?;

        let hashed = match row.try_get::<String, _>("password") {
            Ok(pwd) => pwd,
            Err(_) => return Err(StoreError::UserDataNotFound("password".to_string()))
        };

        let parsed_hash = match PasswordHash::new(&hashed) {
            Ok(ph) => ph,
            Err(_) => return Err(StoreError::MalformedStoreHash),
        };

        if Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok() {
                Ok(true)
            } else {
                Err(StoreError::IncorrectPassword)
            }
    }

    pub async fn register_user (&self, email: &str, password: &str) -> Result<(), StoreError> {
        let password = password.as_bytes();
        let hashed_password = Self::hash_password(password)
            .map_err(|e| StoreError::HashError(e.to_string()))?;
        sqlx::query(
            "INSERT INTO accounts (email, password) VALUES ($1, $2)",
        )
        .bind(email)
        .bind(&hashed_password)
        .execute(&self.connection)
        .await?;

    Ok(())
    }

    fn hash_password(password: &[u8]) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default().hash_password(password, &salt)?.to_string();
        Ok(hashed_password)
    }

    pub async fn create_project(&self, project: Project) -> Result<(), StoreError> {
        // Validate project has a name and valid associated email
        if let Err(e) = project.validate() {
            return Err(StoreError::InvalidInput(format!("{}", e)));
        }
        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO projects (id, user_email, name, description)
            VALUES ($1, $2, $3, $4)"
        )
        .bind(id)
        .bind(project.user_email)
        .bind(project.name)
        .bind(project.description)
        .execute(&self.connection)
        .await?;

    Ok(())
    }

    pub async fn list_projects(&self, user_email: String) -> Result<Vec<Project>, StoreError> {
        let projects = sqlx::query_as!(
            Project,
            "SELECT id, user_email, name, description, created_at, last_updated FROM projects WHERE user_email = $1",
            user_email
        )
        .fetch_all(&self.connection)
        .await?;

        Ok(projects)
    }

    pub async fn get_project_by_id(
        &self, id: uuid::Uuid
    ) -> Result<Project, StoreError> {
        let project = sqlx::query_as!(
            Project,
            r#"
            SELECT id, user_email, name, description, created_at, last_updated
            FROM projects
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.connection)
        .await?;

        Ok(project)
    }

    pub async fn update_project (&self, update: UpdateProject, id: uuid::Uuid)
    -> Result<Project, StoreError> {
        let offset_dt = sqlx::types::time::OffsetDateTime::from_unix_timestamp(update.update_timestamp / 1000)
            .map_err(|e| StoreError::UserDataNotFound(format!("Invalid timestamp: {}", e)))?;
        let timestamp_dt = sqlx::types::time::PrimitiveDateTime::new(offset_dt.date(), offset_dt.time());
        let project = sqlx::query_as!(
            Project,
            r#"UPDATE projects SET name = $1, description = $2, last_updated = $3 WHERE id = $4
            RETURNING id, user_email, name, description, created_at, last_updated"#,
            update.name, update.description, timestamp_dt, id
        )
        .fetch_one(&self.connection)
        .await?;

        Ok(project)
    }

    pub async fn delete_project(&self, id: uuid::Uuid)
    -> Result<(), StoreError> {
        let result = sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(id)
            .execute(&self.connection)
            .await?;

        if result.rows_affected() == 0 {
            return Err(StoreError::ProjectNotFound);
        }

        Ok(())
    }

    pub async fn get_account_settings(&self, user_email: String) -> Result<AccountSettings, StoreError> {
        let account = sqlx::query_as!(
            AccountSettings,
            r#"SELECT email, name FROM accounts WHERE email = $1"#,
            user_email
        )
        .fetch_one(&self.connection)
        .await?;

        Ok(account)
    }

    pub async fn edit_account_settings(&self, user_email: String, update_request: AccountUpdateRequest) -> Result<(), StoreError> {
        // If new password is provided, verify current password first
        if let Some(new_password) = &update_request.new_password {
            if new_password.trim().is_empty() {
                return self.update_name_and_email_only(user_email, update_request).await;
            }

            // Verify current password is provided and correct
            let current_password = update_request.current_password
                .as_ref()
                .ok_or(StoreError::MissingCurrentPassword)?;

            if !self.authenticate_user(&user_email, current_password).await? {
                return Err(StoreError::InvalidCurrentPassword);
            }

            // Update all fields including password
            let hashed_password = Self::hash_password(new_password.as_bytes())
                .map_err(|e| StoreError::HashError(e.to_string()))?;

            let result = sqlx::query("UPDATE accounts SET email = $1, password = $2, name = $3 WHERE email = $4")
                .bind(&update_request.email)
                .bind(hashed_password)
                .bind(&update_request.name)
                .bind(user_email)
                .execute(&self.connection)
                .await?;

            if result.rows_affected() == 0 {
                return Err(StoreError::UserNotFound);
            }
        } else {
            // No password change, update only name and email
            return self.update_name_and_email_only(user_email, update_request).await;
        }

        Ok(())
    }

    async fn update_name_and_email_only(&self, user_email: String, update_request: AccountUpdateRequest) -> Result<(), StoreError> {
        let result = sqlx::query("UPDATE accounts SET email = $1, name = $2 WHERE email = $3")
            .bind(&update_request.email)
            .bind(&update_request.name)
            .bind(user_email)
            .execute(&self.connection)
            .await?;

        if result.rows_affected() == 0 {
            return Err(StoreError::UserNotFound);
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("Password hashing failed: {0}")]
    HashError(String),

    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("User not found")]
    UserNotFound,

    #[error("Project not found")]
    ProjectNotFound,

    #[error("User data not found: {0}")]
    UserDataNotFound(String),

    #[error("Malformed store hash")]
    MalformedStoreHash,

    #[error("Incorrect Password")]
    IncorrectPassword,

    #[error("Failed to create project")]
    FailedProjectCreation,

    #[error("Failed to get account settings")]
    FailedGetAccount,

    #[error("Failed to update account")]
    FailedUpdateAccount,

    #[error("Current password is required when setting a new password")]
    MissingCurrentPassword,

    #[error("Current password is incorrect")]
    InvalidCurrentPassword,

    #[error("Session error")]
    SessionError,

    #[error("Invalid user input: {0}")]
    InvalidInput(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for StoreError {
    fn into_response(self) -> Response {
        let status = match self {
            StoreError::UserNotFound
            | StoreError::IncorrectPassword
            | StoreError::InvalidCurrentPassword
            | StoreError::SessionError  => StatusCode::UNAUTHORIZED,
            StoreError::UserDataNotFound(_)
            | StoreError::ProjectNotFound
            | StoreError::FailedGetAccount
            | StoreError::FailedUpdateAccount
            | StoreError::MissingCurrentPassword
            | StoreError::MalformedStoreHash
            | StoreError::FailedProjectCreation
            | StoreError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            StoreError::HashError(_)
            | StoreError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(ErrorResponse {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}
