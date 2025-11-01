use axum::{
    routing::{post, get, put, delete},
    extract::{State, DefaultBodyLimit, Request},
    http::StatusCode,
    response::IntoResponse,
    Router,
    ServiceExt,
};
use tower_sessions::{Expiry, SessionManagerLayer, cookie::time::Duration};
use std::{sync::Arc};
use sqlx::migrate;
use tower_sessions_sqlx_store::PostgresStore;
use tower::Layer;
use tower_http::{
    compression::CompressionLayer,
    normalize_path::NormalizePathLayer,
    services::{ ServeDir, ServeFile },
};

#[macro_use]
extern crate lazy_static;

mod store;
mod rate_limit;
mod config;
mod auth;
mod project;

use auth::routes::{
  login,
  logout,
  get_current_user,
  add_user,
};
use project::routes::{
  create_project_handler,
  list_project_handler,
  get_project_by_id_handler,
  update_project_handler,
  delete_project_handler,
};


#[tokio::main]
async fn main() {
    dotenvy::from_filename(".env").ok();

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let user_password_db = store::Store::new(&db_url).await;

    migrate!()
        .run(&user_password_db.connection)
        .await
        .expect("Failed to run migrations");

    let shared_store = Arc::new(user_password_db);

    let memory_store = PostgresStore::new(shared_store.connection.clone());
    let session_layer = SessionManagerLayer::new(memory_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(30)));

    let api_routes = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(get_current_user))
        .route("/signup", post(add_user))
        .route("/projects", post(create_project_handler))
        .route("/projects", get(list_project_handler))
        .route("/projects/{id}", get(get_project_by_id_handler))
        .route("/projects/{id}", put(update_project_handler))
        .route("/projects/{id}", delete(delete_project_handler))
        .route("/healthz", get(health_check))
        .with_state(shared_store)
        .layer(session_layer);

    let index = format!("{}{}", config::FRONTEND_PATH.to_string(), "/index.html");
    let serve_dir = ServeDir::new(config::FRONTEND_PATH.to_string()).not_found_service(
        ServeFile::new(index)
    );
    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(serve_dir)
        // Disabled for now, as svelte inlines scripts
        // .layer(middleware::from_fn(csp::add_csp_header))
        .layer(DefaultBodyLimit::max(*config::LIMIT))
        .layer(CompressionLayer::new().br(true).deflate(true).gzip(true).zstd(true));

    let app = NormalizePathLayer::trim_trailing_slash().layer(app);

    let listener = tokio::net::TcpListener::bind(config::LISTEN_ADDR.to_string()).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, ServiceExt::<Request>::into_make_service(app)).await.unwrap();
}

async fn health_check(State(store): State<Arc<store::Store>>) -> impl IntoResponse {
    if let Err(e) = sqlx::query("SELECT 1").execute(&store.connection).await {
        eprintln!("Health check DB error: {:?}", e);
        return StatusCode::SERVICE_UNAVAILABLE;
    }
    StatusCode::OK
}
