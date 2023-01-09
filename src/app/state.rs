use std::marker::{Send, Sync};
use std::sync::Arc;

use sqlx::PgPool;

use crate::interactors::pg::UserUseCaseImpl;
use crate::{infra::pg::PgRepositories, usecases::UserUseCase};

pub trait Context: Send + Sync + 'static {
    type User: UserUseCase;

    fn user(&self) -> &Self::User;
}

pub struct PgContext {
    pub user_usecase: UserUseCaseImpl<PgRepositories>,
}

impl PgContext {
    pub fn new(conn: PgPool) -> Self {
        let repositories = PgRepositories::new();

        let user_usecase = UserUseCaseImpl::new(conn.clone(), repositories);

        Self { user_usecase }
    }
}

impl Context for PgContext {
    type User = UserUseCaseImpl<PgRepositories>;

    fn user(&self) -> &Self::User {
        &self.user_usecase
    }
}

pub type AppState<T> = Arc<T>;
