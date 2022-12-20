use crate::models::User;
use async_trait::async_trait;
use std::marker::{Send, Sync};

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn list_users(&self) -> Result<Vec<User>, sqlx::Error>;
}
