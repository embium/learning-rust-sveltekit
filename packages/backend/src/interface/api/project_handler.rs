use std::sync::Arc;

use axum::{ extract::{State, Path}, middleware, routing::get, Extension, Json, Router };

use crate::{
    application::{dto::project::create_update_project_request::CreateOrUpdateProject, state::AppState},
    domain::entities::{project::{Project, ProjectWithOwnerEmail}, user::UserFull},
    infra::{
        errors::app_error::AppError,
        utils::response::SuccessResponse,
    },
    interface::middleware::auth_mw::is_authorized,
};

pub fn setup_project_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_all_projects_by_user_id).post(create_project))
        .route("/{id}", get(get_project_by_id_and_user_id).put(update_project).delete(delete_project))
        .layer(middleware::from_fn_with_state(app_state.clone(), is_authorized))
}

async fn get_all_projects_by_user_id(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>
) -> Result<SuccessResponse<Vec<Project>>, AppError> {
    let user_id = &current_user.user.id;

    let projects = state.uc.project.get_all_by_user_id.execute(user_id).await?;

    Ok(SuccessResponse::with_data(200, projects))
}

async fn get_project_by_id_and_user_id(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>
) -> Result<SuccessResponse<ProjectWithOwnerEmail>, AppError> {
    let user_id = &current_user.user.id;

    let project = state.uc.project.get_by_id_and_user_id.execute(&project_id, user_id).await?;

    Ok(SuccessResponse::with_data(200, project))
}

async fn create_project(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateOrUpdateProject>
) -> Result<SuccessResponse<Project>, AppError> {
    let user_id = &current_user.user.id;

    let project = state.uc.project.create_project.execute(user_id, req).await?;

    Ok(SuccessResponse::with_data(201, project))
}

async fn update_project(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>,
    Json(req): Json<CreateOrUpdateProject>
) -> Result<SuccessResponse<ProjectWithOwnerEmail>, AppError> {
    let user_id = &current_user.user.id;

    let project = state.uc.project.update_project.execute(&project_id, user_id, req).await?;

    Ok(SuccessResponse::with_data(200, project))
}

async fn delete_project(
    Extension(current_user): Extension<UserFull>,
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<String>,
) -> Result<SuccessResponse<Project>, AppError> {
    let user_id = &current_user.user.id;

    state.uc.project.delete_project.execute(&project_id, user_id).await?;

    Ok(SuccessResponse::with_code(200))
}
