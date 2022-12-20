use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::{models::User, repositories::UserRepository};

pub struct UserRepositoryImpl {
    conn: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(conn: PgPool) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn list_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let sql = "SELECT * from users";
        sqlx::query_as::<_, User>(sql).fetch_all(&self.conn).await
    }
}
