use axum::{
    extract::{Json, State, Path},
    http::StatusCode,
};
use std::{sync::Arc};
use tower_sessions::{Session};

use crate::store;
use crate::rate_limit;
use crate::auth::lib::require_login;

pub async fn create_project_handler(
    State(store): State<Arc<store::Store>>,
    session: Session,
    _rate_limit: rate_limit::RateLimit,
    Json(project): Json<store::Project>,
) -> Result<StatusCode, store::StoreError> {
    require_login(&session).await?;
    store.create_project(project)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| store::StoreError::FailedProjectCreation)
}

pub async fn list_project_handler(
    State(store): State<Arc<store::Store>>,
    session: Session,
    _rate_limit: rate_limit::RateLimit,
) -> Result<Json<Vec<store::Project>>, store::StoreError> {
    let user_email = require_login(&session).await?;
    store.list_projects(user_email)
      .await
      .map(Json)
}

pub async fn get_project_by_id_handler(
    State(store): State<Arc<store::Store>>,
    session: Session,
    _rate_limit: rate_limit::RateLimit,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<store::Project>, store::StoreError> {
    require_login(&session).await?;
    store.get_project_by_id(id)
        .await
        .map(Json)
}

pub async fn update_project_handler(
    State(store): State<Arc<store::Store>>,
    session: Session,
    _rate_limit: rate_limit::RateLimit,
    Path(id): Path<uuid::Uuid>,
    Json(update): Json<store::UpdateProject>
) -> Result<Json<store::Project>, store::StoreError> {
    require_login(&session).await?;
    store.update_project(update, id)
        .await
        .map(Json)
}

pub async fn delete_project_handler(
    State(store): State<Arc<store::Store>>,
    session: Session,
    _rate_limit: rate_limit::RateLimit,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, store::StoreError> {
    require_login(&session).await?;
    store.delete_project(id)
        .await
        .map(|_| StatusCode::OK)
}
