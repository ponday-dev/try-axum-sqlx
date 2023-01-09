mod user;

use sqlx::{pool::PoolConnection, Postgres, Transaction};
pub use user::*;

use crate::repositories::Repositories;

pub struct PgRepositories {
    user_repository: UserRepositoryImpl,
}

impl PgRepositories {
    pub fn new() -> Self {
        let user_repository = UserRepositoryImpl::new();

        Self { user_repository }
    }
}

impl Repositories<PoolConnection<Postgres>, Transaction<'_, Postgres>> for PgRepositories {
    type UserRepo = UserRepositoryImpl;

    fn user(&self) -> &Self::UserRepo {
        &self.user_repository
    }
}
