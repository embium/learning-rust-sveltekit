use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use std::{sync::Arc};
use tower_sessions::{Session};

use crate::store;
use crate::rate_limit;
use crate::auth::lib::require_login;

pub async fn get_account_settings_handler(
    State(store): State<Arc<store::Store>>,
    session: Session,
    _rate_limit: rate_limit::RateLimit,
) -> Result<Json<store::AccountSettings>, store::StoreError> {
    let user_email = require_login(&session).await?;
    store.get_account_settings(user_email)
        .await
        .map(Json)
        .map_err(|_| store::StoreError::FailedGetAccount)
}

pub async fn edit_account_settings_handler(
    State(store): State<Arc<store::Store>>,
    session: Session,
    _rate_limit: rate_limit::RateLimit,
    Json(update_request): Json<store::AccountUpdateRequest>,
) -> Result<StatusCode, store::StoreError> {
    let user_email = require_login(&session).await?;
    store.edit_account_settings(user_email, update_request)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| store::StoreError::FailedUpdateAccount)
}
