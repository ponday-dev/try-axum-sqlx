use async_trait::async_trait;

use crate::{
    app::error::AppError,
    models::{CreateUserDto, User},
};
use std::marker::{Send, Sync};

#[async_trait]
pub trait UserUseCase: Send + Sync + 'static {
    async fn list_users(&self) -> anyhow::Result<Vec<User>, AppError>;
    async fn create_user(&self, user: CreateUserDto) -> anyhow::Result<User, AppError>;
}
