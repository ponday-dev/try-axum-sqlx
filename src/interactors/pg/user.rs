use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{pool::PoolConnection, PgPool, Postgres, Transaction};

use crate::{
    app::error::AppError,
    models::{CreateUserDto, User},
    repositories::{Repositories, UserRepository},
    usecases::UserUseCase,
};

pub struct UserUseCaseImpl<R>
where
    for<'a> R: Repositories<PoolConnection<Postgres>, Transaction<'a, Postgres>>,
{
    conn: PgPool,
    repos: Arc<R>,
}

impl<R> UserUseCaseImpl<R>
where
    for<'a> R: Repositories<PoolConnection<Postgres>, Transaction<'a, Postgres>>,
{
    pub fn new(conn: PgPool, repositories: R) -> Self {
        Self {
            conn,
            repos: Arc::new(repositories),
        }
    }
}

#[async_trait]
impl<R> UserUseCase for UserUseCaseImpl<R>
where
    for<'a> R: Repositories<PoolConnection<Postgres>, Transaction<'a, Postgres>>,
{
    async fn list_users(&self) -> anyhow::Result<Vec<User>, AppError> {
        let mut conn = self.conn.acquire().await?;

        self.repos.user().list_users(&mut conn).await
    }

    async fn create_user(&self, user: CreateUserDto) -> anyhow::Result<User, AppError> {
        let mut tx = self.conn.begin().await?;

        let result = self.repos.user().create_user(&mut tx, user).await;

        match result {
            Ok(user) => {
                tx.commit().await?;
                Ok(user)
            }
            Err(err) => {
                tx.rollback().await?;
                Err(err)
            }
        }
    }
}
