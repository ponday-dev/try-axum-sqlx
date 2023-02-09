use async_trait::async_trait;
use sqlx::{pool::PoolConnection, Postgres, Transaction};

use crate::{
    app::error::AppError,
    models::{CreateUserDto, User},
    repositories::UserRepository,
};

pub struct UserRepositoryImpl;

impl UserRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl UserRepository<PoolConnection<Postgres>, Transaction<'_, Postgres>> for UserRepositoryImpl {
    async fn list_users(
        &self,
        conn: &mut PoolConnection<Postgres>,
    ) -> anyhow::Result<Vec<User>, AppError> {
        let sql = "SELECT * from users";
        let result = sqlx::query_as::<_, User>(sql).fetch_all(&mut *conn).await;

        match result {
            Ok(users) => Ok(users),
            Err(e) => Err(AppError::QueryError(e)),
        }
    }

    async fn create_user(
        &self,
        tx: &mut Transaction<Postgres>,
        data: CreateUserDto,
    ) -> anyhow::Result<User, AppError> {
        let sql = "INSERT INTO users (name) VALUES ($1) returning *";
        let result = sqlx::query_as::<_, User>(sql)
            .bind(&data.name)
            .fetch_one(&mut *tx)
            .await;

        match result {
            Ok(user) => Ok(user),
            Err(e) => Err(AppError::QueryError(e)),
        }
    }
}
