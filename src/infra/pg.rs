mod user;

use sqlx::PgPool;
pub use user::*;

use crate::repositories::Repositories;

pub struct PgRepositories {
    user_repository: UserRepositoryImpl,
}

impl PgRepositories {
    pub fn new(conn: PgPool) -> Self {
        let user_repository = UserRepositoryImpl::new(conn.clone());

        Self { user_repository }
    }
}

impl Repositories for PgRepositories {
    type UserRepo = UserRepositoryImpl;

    fn user(&self) -> &Self::UserRepo {
        &self.user_repository
    }
}
