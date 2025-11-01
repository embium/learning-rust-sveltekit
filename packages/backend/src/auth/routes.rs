use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use std::{sync::Arc};
use validator::Validate;
use tower_sessions::{Session};

use crate::store;
use crate::rate_limit;
use crate::auth::lib::require_login;

pub async fn login(
    State(store): State<Arc<store::Store>>,
    session: Session,
    _rate_limit: rate_limit::RateLimit,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, store::StoreError> {
    store.authenticate_user(&payload.email, &payload.password).await?;

    session.insert("user_email", &payload.email).await
        .map_err(|_| store::StoreError::SessionError)?;

    Ok(Json(serde_json::json!({
        "email": payload.email,
        "message": "Login successful"
    })))
}

pub async fn logout(
    session: Session,
    _rate_limit: rate_limit::RateLimit,
) -> Result<impl IntoResponse, store::StoreError> {
    session.clear().await;

    Ok(Json(serde_json::json!({
        "message": "Logout successful"
    })))
}

pub async fn get_current_user(
    session: Session,
    _rate_limit: rate_limit::RateLimit,
) -> Result<impl IntoResponse, store::StoreError> {
    let user_email = require_login(&session).await?;

    Ok(Json(serde_json::json!({
        "email": user_email,
        "authenticated": true
    })))
}

pub async fn add_user(
    State(store): State<Arc<store::Store>>,
    Json(payload): Json<NewUser>,
) -> Result<impl IntoResponse, store::StoreError> {
    // Validate the payload user data
    if let Err(e) = payload.validate() {
        return Err(store::StoreError::InvalidInput(format!("{}", e)));
    }
    match store.register_user(&payload.email, &payload.password).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            eprintln!("Signup error: {:?}", e);
            Err(e)
        }
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize, Validate)]
pub struct NewUser {
    #[validate(email)]
    email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    password: String,
}
