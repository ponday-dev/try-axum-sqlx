use crate::models::User;
use async_trait::async_trait;
use std::marker::{Send, Sync};

pub struct CreateUserDto {
    pub name: String,
}

#[async_trait]
pub trait UserRepository<Conn, Tran>: Send + Sync + 'static {
    async fn list_users(&self, ctx: &mut Conn) -> anyhow::Result<Vec<User>, sqlx::Error>;
    async fn create_user(
        &self,
        ctx: &mut Tran,
        data: CreateUserDto,
    ) -> anyhow::Result<User, sqlx::Error>;
}
