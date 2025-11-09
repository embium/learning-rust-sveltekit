use std::sync::Arc;

use super::update_user_settings::UpdateUserSettingsUseCase;
use super::get_user_settings::GetUserSettingsUseCase;
use crate::infra::repositories::pg_user_repo::PgUserRepository;
use sqlx::PgPool;

pub struct UserUseCases {
    pub update_user_settings: UpdateUserSettingsUseCase,
    pub get_user_settings: GetUserSettingsUseCase,
}

impl UserUseCases {
    pub fn new(user_repo: Arc<PgUserRepository>, db_pool: PgPool) -> Self {
        Self {
            update_user_settings: UpdateUserSettingsUseCase::new(user_repo.clone(), db_pool.clone()),
            get_user_settings: GetUserSettingsUseCase::new(user_repo.clone()),
        }
    }
}
