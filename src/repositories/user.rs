use crate::{
    app::error::AppError,
    models::{CreateUserDto, User},
};
use async_trait::async_trait;
use std::marker::{Send, Sync};

#[async_trait]
pub trait UserRepository<Conn, Tran>: Send + Sync + 'static {
    async fn list_users(&self, ctx: &mut Conn) -> anyhow::Result<Vec<User>, AppError>;
    async fn create_user(
        &self,
        ctx: &mut Tran,
        data: CreateUserDto,
    ) -> anyhow::Result<User, AppError>;
}
