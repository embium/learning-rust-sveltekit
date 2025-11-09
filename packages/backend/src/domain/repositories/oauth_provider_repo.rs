use crate::{
    domain::entities::user_oauth_provider::UserOauthProvider, infra::errors::app_error::AppError,
};

#[async_trait::async_trait]
pub trait OauthProviderRepository {
    async fn get_by_provider_and_id(
        &self,
        provider: &str,
        provider_id: &str,
    ) -> Result<UserOauthProvider, AppError>;

    async fn get_by_user_id_and_provider(
        &self,
        user_id: &str,
        provider: &str,
    ) -> Result<UserOauthProvider, AppError>;

    // async fn get_user_with_provider_by_user_id(&self, user_id: &str) -> Result<UserFull, AppError>;
}
