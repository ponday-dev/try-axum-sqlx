use anyhow::Result;
use async_trait::async_trait;

use crate::models::User;
use std::marker::{Send, Sync};

#[async_trait]
pub trait UserUseCase: Send + Sync + 'static {
    async fn list_users(&self) -> Result<Vec<User>>;
}
