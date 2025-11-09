use std::sync::Arc;
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method
    },
    extract::Request,
    Router,
    ServiceExt,
};

use casbin::{ CoreApi, DefaultModel, Enforcer };
use sqlx_adapter::SqlxAdapter;
use tokio::sync::RwLock;
use tower_http::cors::{ AllowOrigin, CorsLayer };
use tracing::{ debug, info };
use tower::Layer;
use tower_http::{
    normalize_path::NormalizePathLayer
};

use crate::{
    application::state::AppState,
    infra::{ graceful::shutdown_signal, rbac::Rbac },
    interface::api::{
        auth_handler::setup_auth_routes,
        permission_handler::setup_permission_handler,
        public_oauth_handler::setup_public_oauth_handler,
        role_handler::setup_role_routes,
        super_handler::setup_super_handler,
        project_handler::setup_project_routes,
        user_handler::setup_user_routes,
    },
};

use super::{ config::AppConfig, data::{ postgres::establish_connection, redis::get_redis_con } };

#[derive(Debug)]
pub struct ServerBuilder {
    cfg: Arc<AppConfig>,
}

impl ServerBuilder {
    pub fn new(cfg: Arc<AppConfig>) -> Self {
        Self { cfg }
    }

    pub async fn run(&self) {
        let db_pool = establish_connection(&self.cfg.db_url).await;

        /*if &self.cfg.app_env != "local" {
            info!("Running Automigrations for NON-LOCAL ENV");

            sqlx::migrate("./migrations").run(&db_pool).await.expect("Failed to run migrations");

            info!("Automigrations Completed!");
        }*/

        let redis_pool = get_redis_con(&self.cfg.redis_url).await;

        // casbin enforcer
        let enforcer = Arc::new(RwLock::new(self.setup_casbin().await));

        // setup roles & permissions casbin rbac
        let rbac = Arc::new(Rbac::new(enforcer));
        // rbac.setup_roles_and_permissions().await; // not used anymore

        let app_state = Arc::new(AppState::new(self.cfg.clone(), db_pool, redis_pool, rbac));

        let api_routes = self.setup_api_router(app_state.clone());

        let app_router = Router::new()
            .nest("/oauth", setup_public_oauth_handler())
            .nest("/api", api_routes)
            .layer(self.setup_cors())
            .with_state(app_state);

        let app = NormalizePathLayer::trim_trailing_slash().layer(app_router);

        // Run Server
        let addr = format!("0.0.0.0:{}", &self.cfg.app_port);
        let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind address");

        debug!("🚀 API Started on {}", addr);
        axum::serve(listener, ServiceExt::<Request>::into_make_service(app)).with_graceful_shutdown(shutdown_signal()).await
        .expect("API Server Error");
    }

    fn setup_api_router(&self, app_state: Arc<AppState>) -> Router<Arc<AppState>> {
        Router::new()
            .nest("/v1/permissions", setup_permission_handler())
            .nest("/v1/roles", setup_role_routes(app_state.clone()))
            .nest("/v1/auth", setup_auth_routes(app_state.clone()))
            .nest("/v1/super", setup_super_handler(app_state.clone()))
            .nest("/v1/projects", setup_project_routes(app_state.clone()))
            .nest("/v1/user", setup_user_routes(app_state.clone()))
    }

    fn setup_cors(&self) -> CorsLayer {
        let origins = self.cfg.allowed_origins.clone();

        let allowed_origins: Vec<HeaderValue> = origins
            .split(',')
            .filter_map(|origin| origin.parse::<HeaderValue>().ok())
            .collect();

        let allow_origin = AllowOrigin::list(allowed_origins);

        CorsLayer::new()
            .allow_origin(allow_origin)
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PATCH,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_credentials(true)
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
    }

    async fn setup_casbin(&self) -> Enforcer {
        // casbin config initialization
        let model = DefaultModel::from_file("etc/rbac_model.conf").await.unwrap();
        let adapter = SqlxAdapter::new(&self.cfg.db_url, 8).await.unwrap();

        Enforcer::new(model, adapter).await.unwrap()
    }
}
