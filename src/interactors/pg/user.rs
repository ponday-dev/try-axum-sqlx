use std::sync::Arc;

use anyhow::bail;
use async_trait::async_trait;
use sqlx::{pool::PoolConnection, PgPool, Postgres, Transaction};

use crate::{
    models::User,
    repositories::{CreateUserDto, Repositories, UserRepository},
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
    async fn list_users(&self) -> anyhow::Result<Vec<User>> {
        let mut conn = self.conn.acquire().await?;

        let result = self.repos.user().list_users(&mut conn).await;

        match result {
            Ok(users) => Ok(users),
            Err(err) => bail!(err),
        }
    }

    async fn create_user(&self) -> anyhow::Result<User> {
        let mut tx = self.conn.begin().await?;

        let data = CreateUserDto {
            name: "Test".to_string(),
        };
        let result = self.repos.user().create_user(&mut tx, data).await;

        match result {
            Ok(user) => {
                tx.commit().await?;
                Ok(user)
            }
            Err(err) => {
                tx.rollback().await?;
                bail!(err)
            }
        }
    }
}
