use tower_sessions::{Session};
use crate::store;

pub async fn require_login(session: &Session) -> Result<String, store::StoreError> {
    session.get::<String>("user_email").await
        .map_err(|_| store::StoreError::SessionError)?
        .ok_or(store::StoreError::IncorrectPassword)
}
