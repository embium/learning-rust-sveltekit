use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    middleware,
    Extension, Json, Router,
};

use crate::{
    application::{
        dto::auth::user_settings_dto::{UserSettingsDto, UserSettingsUpdateDto},
        state::AppState,
    },
    domain::entities::user::UserFull,
    infra::{errors::app_error::AppError, utils::response::SuccessResponse},
    interface::middleware::auth_mw::is_authorized,
};

pub fn setup_user_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/settings", get(get_user_settings).put(update_user_settings))
        .layer(middleware::from_fn_with_state(app_state.clone(), is_authorized))
}

pub async fn update_user_settings(
    State(app_state): State<Arc<AppState>>,
    Extension(current_user): Extension<UserFull>,
    Json(update_dto): Json<UserSettingsUpdateDto>,
) -> Result<SuccessResponse<UserSettingsDto>, AppError> {
    let updated_user = app_state
        .uc
        .user
        .update_user_settings
        .execute(&current_user.user.id, update_dto)
        .await?;

    Ok(SuccessResponse::with_data(StatusCode::OK.as_u16(), updated_user))
}

pub async fn get_user_settings(
    State(app_state): State<Arc<AppState>>,
    Extension(current_user): Extension<UserFull>,
) -> Result<SuccessResponse<UserSettingsDto>, AppError> {
    let user_settings = app_state
        .uc
        .user
        .get_user_settings
        .execute(&current_user.user.id)
        .await?;

    Ok(SuccessResponse::with_data(StatusCode::OK.as_u16(), user_settings))
}
