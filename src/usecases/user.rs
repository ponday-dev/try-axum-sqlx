use async_trait::async_trait;

use crate::models::User;
use std::marker::{Send, Sync};

#[async_trait]
pub trait UserUseCase: Send + Sync + 'static {
    async fn list_users(&self) -> anyhow::Result<Vec<User>>;
    async fn create_user(&self) -> anyhow::Result<User>;
}
