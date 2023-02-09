use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Query execution failed.")]
    QueryError(#[from] sqlx::Error),
}
