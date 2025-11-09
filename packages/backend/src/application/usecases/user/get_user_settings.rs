use std::sync::Arc;

use crate::{
    application::dto::auth::user_settings_dto::UserSettingsDto,
    domain::repositories::user_repo::UserRepository,
    infra::{errors::app_error::AppError, repositories::pg_user_repo::PgUserRepository},
};

pub struct GetUserSettingsUseCase {
    pub user_repo: Arc<PgUserRepository>,
}

impl GetUserSettingsUseCase {
    pub fn new(user_repo: Arc<PgUserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(
        &self,
        user_id: &str,
    ) -> Result<UserSettingsDto, AppError> {
        let user = self.user_repo.find_by_id(user_id).await?;

        let provider = self.user_repo.find_provider_by_user_id(user_id).await?;

        Ok(UserSettingsDto {
          fullname: user.fullname,
          email: user.email,
          provider: provider.provider,
        })
    }
}
