use std::sync::Arc;

use anyhow::{bail, Result};
use async_trait::async_trait;

use crate::{
    models::User,
    repositories::{Repositories, UserRepository},
    usecases::UserUseCase,
};

pub struct UserUseCaseImpl<R: Repositories> {
    repos: Arc<R>,
}

impl<R: Repositories> UserUseCaseImpl<R> {
    pub fn new(repositories: R) -> Self {
        Self {
            repos: Arc::new(repositories),
        }
    }
}

#[async_trait]
impl<R: Repositories> UserUseCase for UserUseCaseImpl<R> {
    async fn list_users(&self) -> Result<Vec<User>> {
        let result = self.repos.user().list_users().await;

        match result {
            Ok(users) => Ok(users),
            Err(err) => bail!(err),
        }
    }
}
