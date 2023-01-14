use async_trait::async_trait;
use sqlx::{pool::PoolConnection, Postgres, Transaction};

use crate::{
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
    ) -> anyhow::Result<Vec<User>, sqlx::Error> {
        let sql = "SELECT * from users";
        sqlx::query_as::<_, User>(sql).fetch_all(&mut *conn).await
    }

    async fn create_user(
        &self,
        tx: &mut Transaction<Postgres>,
        data: CreateUserDto,
    ) -> anyhow::Result<User, sqlx::Error> {
        let sql = "INSERT INTO users (name) VALUES ($1) returning *";
        sqlx::query_as::<_, User>(sql)
            .bind(&data.name)
            .fetch_one(&mut *tx)
            .await
    }
}
