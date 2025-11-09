use std::sync::Arc;

use crate::{
    application::dto::auth::user_settings_dto::{UserSettingsDto, UserSettingsUpdateDto},
    domain::repositories::user_repo::UserRepository,
    infra::{errors::app_error::AppError, repositories::pg_user_repo::PgUserRepository, utils::password::{verify_password, hash_password}},
};

pub struct UpdateUserSettingsUseCase {
    pub user_repo: Arc<PgUserRepository>,
    pub db_pool: sqlx::PgPool,
}

impl UpdateUserSettingsUseCase {
    pub fn new(user_repo: Arc<PgUserRepository>, db_pool: sqlx::PgPool) -> Self {
        Self { user_repo, db_pool }
    }

    pub async fn execute(
        &self,
        user_id: &str,
        update_dto: UserSettingsUpdateDto,
    ) -> Result<UserSettingsDto, AppError> {
        let mut tx = self.db_pool.begin().await?;

        // Fetch the current user
        let mut user = self.user_repo.find_by_id(user_id).await?;

        let provider = self.user_repo.find_provider_by_user_id(user_id).await?;

        // Update user fields
        if let Some(email) = update_dto.email {
          if provider.provider == "email" {
            user.change_email(email);
          }else{
            ()
          }
        }

        if let Some(current_password) = update_dto.current_password && let Some(new_password) = update_dto.new_password {
          if verify_password(user.password_hash.as_ref().unwrap(), current_password.as_bytes()).is_ok() {
            let password_hash = hash_password(new_password.as_bytes()).unwrap();
            user.change_password(Some(password_hash));
          } else {
            ()
          }
        }

        user.update(update_dto.name, None); // avatar_url will be handled separately when needed

        // Save the updated user
        let updated_user = self.user_repo.tx_update(&mut tx, &user).await?;

        tx.commit().await?;

        Ok(UserSettingsDto {
            fullname: updated_user.fullname,
            email: updated_user.email,
            provider: provider.provider,
        })
    }
}
